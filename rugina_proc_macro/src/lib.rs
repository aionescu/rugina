use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
  let ident_str = ident.to_string();

  let new_str = match ident_str.as_str() {
    "Eșuat" => "Err",
    "Bun" => "Ok",
    "Sfoară" => "String",
    "Dicționar" => "HashMap",
    "Implicit" => "Default",
    "Eroare" => "Error",
    "Opțiune" => "Option",
    "Ceva" => "Some",
    "Nimic" => "None",
    "Rezultat" => "Result",
    "Sine" => "Self",
    "afișează_linie" => "println",
    "sparge" => "break",
    "asincronă" => "async",
    "așteaptă" => "await",
    "buclă" => "loop",
    "mișcă" => "move",
    "ladă" => "crate",
    "cod_inaccesibil" => "unreachable_code",
    "ca" => "as",
    "constant" => "const",
    "trăsătură" => "trait",
    "nesigur" | "prietenii_știu_de_ce" => "unsafe",
    "în" => "in",
    "din" => "from",
    "dinamic" => "dyn",
    "desfă" => "unwrap",
    "implicit" => "default",
    "ca_referință" => "as_ref",
    "intrare_ieșire" => "io",
    "externă" => "extern",
    "fals" => "false",
    "funcție" => "fn",
    "superior" => "super",
    "inserează" => "insert",
    "obține" => "get",
    "permite" => "allow",
    "futu_i" | "plm" | "hopa" => "panic",
    "modul" => "mod",
    "mutabil" => "mut",
    "nou" => "new",
    "unde" => "where",
    "pentru" => "for",
    "obține_sau_inserează_cu" => "get_or_insert_with",
    "principal" => "main",
    "public" => "pub",
    "returnează" => "return",
    "implementează" => "impl",
    "referință" => "ref",
    "potrivește" => "match",
    "dacă" => "if",
    "altfel" => "else",
    "sine" => "self",
    "fie" => "let",
    "static" => "static",
    "structură" => "struct",
    "dorește" => "expect",
    "cât" => None?, // Used for `cat timp`
    "timp" => "while",
    "folosește" => "use",
    "înspre" => "into",
    "adevărat" => "true",
    "enumerație" => "enum",
    "bibliotecă" => "std",
    "colecții" => "collections",

    _ => &ident_str,
  };

  let new_ident = Ident::new(new_str, ident.span());
  Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
  match tok {
    TokenTree::Group(group) => {
      let mut group_elem = Vec::new();
      replace_stream(group.stream(), &mut group_elem);
      let mut new_stream = TokenStream::new();
      new_stream.extend(group_elem);
      out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
    }
    TokenTree::Ident(ident) => {
      if let Some(ident) = replace_ident(ident) {
        out.push(ident);
      }
    }
    TokenTree::Punct(..) | TokenTree::Literal(..) => {
      out.push(tok);
    }
  }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
  for tok in ts {
    replace_tree(tok, out)
  }
}

#[proc_macro]
pub fn rugină(item: TokenStream) -> TokenStream {
  let mut returned = Vec::new();
  replace_stream(item, &mut returned);
  let mut out = TokenStream::new();
  out.extend(returned);
  out
}
