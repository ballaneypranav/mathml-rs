use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, Result, Token, Type};

mod kw {
    syn::custom_keyword!(to);
    syn::custom_keyword!(with);
    syn::custom_keyword!(into);
}

#[proc_macro]
pub fn attach(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AttachInput);

    let tag = &input.tag;
    let parent_field = input.tag.to_string().to_case(Case::Snake);
    let parent_field_ident = Ident::new(&parent_field, Span::call_site());
    let parent = &input.parents[0];

    let tokens = quote! {
        match container[current] {
            Tag::#parent (ref mut parent) => {
                let #parent_field_ident = #tag::new();
                new_tag = Some(Tag::#tag(#parent_field_ident));
                current = container_len;
                parent.#parent_field_ident = Some(current.clone());
                stack.push(current.clone());
            }
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct AttachInput {
    tag: Ident,
    parents: Vec<Ident>,
    attrs: Vec<Ident>,
}

impl Parse for AttachInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag = syn::Ident::parse(input)?;
        let lookahead = input.lookahead1();
        let mut attrs = Vec::new();
        if lookahead.peek(kw::with) {
            let _with = input.parse::<kw::with>()?;
            let punctuated_attrs = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;
            attrs = punctuated_attrs.into_iter().collect();
        }
        let _to = input.parse::<kw::to>()?;
        let parent = syn::Ident::parse(input)?;
        Ok(AttachInput {
            tag,
            parents: vec![parent],
            attrs,
        })
    }
}

#[proc_macro]
pub fn push(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PushInput);
    //println!("{:?}", input);

    let tag = &input.tag;
    let tag_str = input.tag.to_string();
    let parents = &input.parents;

    // attributes field names and types
    let attr_idents = input.attr_idents;
    let attr_types = input.attr_types;

    // also need strings for matching tokens
    let mut attr_str: Vec<String> = Vec::new();
    for ident in &attr_idents {
        attr_str.push(String::from(ident.to_string()));
    }

    let tokens = quote! {
        // match the current tag
        match container[current] {
            // with the parent
            // TODO: repeat for multiple possible parents
            #(MathNode::#parents (ref mut parent) => {
                // parse any attributes, keeping their types in mind
                //let attributes = e.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                //println!("{:?}", attributes);
                //for attribute in attributes {
                    //let key = str::from_utf8(attribute.key).unwrap();
                    //let value = attribute.unescape_and_decode_value(&reader).unwrap();
                    //match key {
                        //#(#attr_str => {
                            //#parent_field_ident.#attr_idents =
                                //Some(value.parse::<#attr_types>().expect("Incorrect type"));
                        //})*
                        //_ => {}
                    //}
                //}

                // create Tag enum object
                new_tag = Some(MathNode::#tag(#tag::default()));
                // update current pointer (which is really an int)
                current = container_len;
                // update parent pointer of new tag
                parent.children.push(current.clone());
                // push current pointer to stack
                stack.push(current.clone());
                //println!("Opened {}", #tag_str);
            })*
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct PushInput {
    tag: Ident,
    parents: Vec<Ident>,
    attr_idents: Vec<Ident>,
    attr_types: Vec<Type>,
}

impl Parse for PushInput {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("{:#?}", input);
        // parse tag
        let tag = syn::Ident::parse(input)?;
        // define fields used later
        let mut attr_idents = Vec::new();
        let mut attr_types = Vec::new();
        // define lookahead function
        let mut lookahead = input.lookahead1();

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
                if lookahead.peek(kw::into) {
                    break;
                }
            }
        }
        let _into = input.parse::<kw::into>()?;

        // parse parent
        let mut parents = Vec::new();
        parents.push(syn::Ident::parse(input)?);

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

        Ok(PushInput {
            tag,
            parents,
            attr_idents,
            attr_types,
        })
    }
}

#[proc_macro]
pub fn add_op(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as OpInput);
    //println!("{:?}", input);

    let tag = &input.tag;
    let tag_str = input.tag.to_string();
    let parent = &input.parents[0];

    let tokens = quote! {
        // match the current tag
        match container[current] {
            // with the parent
            // TODO: repeat for multiple possible parents
            MathNode::#parent (ref mut parent) => {
                // create Tag enum object
                new_tag = Some(MathNode::new_op(Op::#tag));
                // update current pointer (which is really an int)
                current = container_len;
                // update parent pointer of new tag
                parent.children.push(current.clone());
                // push current pointer to stack
                stack.push(current.clone());
                //println!("Opened {}", #tag_str);
            }
            _ => {}
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct OpInput {
    tag: Ident,
    parents: Vec<Ident>,
}

impl Parse for OpInput {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("{:#?}", input);
        // parse tag
        let tag = syn::Ident::parse(input)?;
        let _to = input.parse::<kw::to>()?;

        // parse parent
        let parent = syn::Ident::parse(input)?;

        Ok(OpInput {
            tag,
            parents: vec![parent],
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
                println!("Trying to close {} but currently in {:?}", #tag_str, container[current]);
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
