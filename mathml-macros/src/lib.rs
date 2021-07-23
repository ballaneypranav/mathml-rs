use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Result, Token, Type};

mod kw {
    syn::custom_keyword!(to);
    syn::custom_keyword!(with);
}

#[proc_macro]
pub fn attach(input: TokenStream) -> TokenStream {
    // parse input
    let input = parse_macro_input!(input as OpenInput);

    // create new node
    let instantiation_expr;
    let pass_object_expr;
    let tag_str;
    let tag_type;
    match input.tag {
        Tag::Ident(tag) => {
            instantiation_expr = quote! {
                let mut new_node = #tag::default();
            };
            pass_object_expr = quote! {
                new_tag = Some(MathNode::#tag(new_node));
            };

            tag_str = tag.to_string();
            tag_type = tag;
        }
        Tag::Enum(enum_name, enum_type) => {
            let enum_str = enum_name.to_string().to_lowercase();
            let fn_str = format!("new_{}", enum_str);
            let fn_ident = Ident::new(&fn_str, Span::call_site());

            instantiation_expr = quote! {};

            pass_object_expr = quote! {
                new_tag = Some(MathNode::#fn_ident(#enum_name::#enum_type));
            };

            tag_str = format!("{}::{}", enum_name.to_string(), enum_type.to_string());
            tag_type = enum_name;
        }
    }

    // attributes field names and types
    let attr_idents = input.attr_idents;
    let attr_types = input.attr_types;
    // also need strings for matching tokens
    let mut attr_str = Vec::new();
    for ident in &attr_idents {
        let mut ident_str = ident.to_string();
        if ident_str.starts_with("r#") {
            ident_str = ident_str.trim_start_matches("r#").to_string()
        }
        attr_str.push(ident_str);
    }

    // create code to parse attributes
    let store_attr = quote! {
        // parse any attributes, keeping their types in mind
        let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
        //println!("{:?}", attributes);
        for attribute in attributes {
            let key = std::str::from_utf8(attribute.key).unwrap();
            let value = attribute.unescape_and_decode_value(&reader).unwrap();
            match key {
                #(#attr_str => {
                    new_node.#attr_idents =
                        Some(value.parse::<#attr_types>().expect("Incorrect type"));
                })*
                _ => {
                    //println!("{:?}", #attr_str);
                    panic!("Attribute {} not parsed for {}", key, #tag_str);
                }
            }
        }
    };

    let index_expr = quote! {
        parent.index(MathNodeType::#tag_type, current.clone());
    };

    let parents = &input.parents;
    // create strings for debugging
    let mut parent_strs: Vec<String> = Vec::new();
    let mut index_exprs: Vec<proc_macro2::TokenStream> = Vec::new();
    // TODO: Convert this to trait
    let parents_to_index: Vec<String> = vec!["Apply", "Lambda", "Piecewise", "Piece", "Otherwise"]
        .iter()
        .map(|&a| a.into())
        .collect();
    for parent in parents {
        let parent_str = parent.to_string();
        if parents_to_index.contains(&parent_str) {
            index_exprs.push(index_expr.clone());
        } else {
            index_exprs.push(quote! {})
        }
        parent_strs.push(parent_str);
    }

    let tokens = quote! {
        {
            // create new object
            #instantiation_expr
            #store_attr
            // match the current tag
            match container[current] {
                // with the parent
                #(MathNode::#parents (ref mut parent) => {
                    #pass_object_expr
                    // update current pointer (which is really an int)
                    current = container_len;
                    // update parent pointer of new tag
                    parent.children.push(current.clone());
                    #index_exprs
                    // push current pointer to stack
                    stack.push(current.clone());
                    //println!("Opened {}", #tag_str);
                })*
                _ => {
                    panic!("Tag {:?} not parsed under parent {:?}", #tag_str, container[current]);
                }
            }
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct OpenInput {
    tag: Tag,
    parents: Vec<Ident>,
    attr_idents: Vec<Ident>,
    attr_types: Vec<Type>,
}

#[derive(Debug)]
enum Tag {
    Ident(Ident),
    Enum(Ident, Ident),
}

impl Parse for OpenInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse tag
        let tag_ident = syn::Ident::parse(input)?;
        let tag: Tag;
        // see if this is an enum
        let mut lookahead = input.lookahead1();
        if lookahead.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            input.parse::<Token![:]>()?;

            let enum_type = syn::Ident::parse(input)?;
            tag = Tag::Enum(tag_ident, enum_type);
        } else {
            tag = Tag::Ident(tag_ident);
        }

        // define fields used later
        let mut attr_idents = Vec::new();
        let mut attr_types = Vec::new();
        // define lookahead function
        lookahead = input.lookahead1();

        // if attributes are specified
        if lookahead.peek(kw::with) {
            let _with = input.parse::<kw::with>()?;

            // loop over attributes and types
            loop {
                // parse attribute field name as ident
                let ident = syn::Ident::parse(input)?;
                attr_idents.push(ident);
                let _as = input.parse::<Token![as]>();
                // parse attribute type
                let ty = syn::Type::parse(input)?;
                attr_types.push(ty);

                // consume comma if it exists
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }

                // break if found into
                // lookahead works only once
                lookahead = input.lookahead1();
                if lookahead.peek(kw::to) {
                    break;
                }
            }
        }
        let _to = input.parse::<kw::to>()?;

        // parse parent
        let mut parents = vec![syn::Ident::parse(input)?];

        // see if there are multiple parents
        loop {
            lookahead = input.lookahead1();
            if lookahead.peek(Token![|]) {
                input.parse::<Token![|]>()?;
            } else {
                break;
            }

            lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                parents.push(syn::Ident::parse(input)?);
            }
        }

        //println!("Parents: {:?}", parents);

        Ok(OpenInput {
            tag,
            parents,
            attr_idents,
            attr_types,
        })
    }
}

#[proc_macro]
pub fn close(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CloseInput);
    //println!("{:?}", input);

    let tag = &input.tag;
    let tag_str = input.tag.to_string();

    let tokens = quote! {
        match container[current] {
            MathNode::#tag (ref mut tag_field) => {
                stack.pop();
                current = stack.last().unwrap().to_owned();
                tag_field.parent = Some(current.clone());
                //println!("Closing {}", #tag_str);
            }
            _ => {
                //println!("{:#?}", container);
                panic!("Trying to close {} but currently in {:?}", #tag_str, container[current]);
            }
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct CloseInput {
    tag: Ident,
}

impl Parse for CloseInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = syn::Ident::parse(input)?;
        Ok(CloseInput { tag })
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
