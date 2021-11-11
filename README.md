# rugină

Aren't you _obosit_ from writing Rust programs in English? Do you like saying
_futu-i_ or _plm_ a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Romanian touch to your
programs?

**rugină** (Romanian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Romanian, using Romanian keywords, Romanian function names,
Romanian idioms.

This has been designed to be used as the official programming language to
develop the future Romanian sovereign operating system.

You don't feel at ease using only Romanian words? Don't worry!
Romanian Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rugină:

## `trăsătură` și `implementează` (a.k.a. `trait` and `impl`)

```rust
rugina::rugină! {
  folosește bibliotecă::colecții::Dicționar ca Dicț;

  trăsătură CheieValoare {
    funcție scrie(&sine, cheie: Sfoară, valoare: Sfoară);
    funcție citește(&sine, cheie: Sfoară) -> Rezultat<Opțiune<&Sfoară>, Sfoară>;
  }

  static mutabil DICȚIONAR: Opțiune<Dicț<Sfoară, Sfoară>> = Nimic;

  structură Concret;

  implementează CheieValoare pentru Concret {
    funcție scrie(&sine, cheie: Sfoară, valoare: Sfoară) {
      fie dicț = nesigur {
        DICȚIONAR.obține_sau_inserează_cu(Implicit::implicit)
      };

      dicț.inserează(cheie, valoare);
    }

    funcție citește(&sine, cheie: Sfoară) -> Rezultat<Opțiune<&Sfoară>, Sfoară> {
      dacă fie Ceva(dicț) = nesigur { DICȚIONAR.ca_referință() } {
        Bun(dicț.obține(&cheie))
      } altfel {
        Ero("Verifica dicționarul".înspre())
      }
    }
  }
}
```

## Alternative syntax

```rust
#[permite(cod_inaccesibil)]
funcție secundar() {
  futu_i!("Futu-i!"); // For the true Romanian experience
  plm!("Vai de plm"); // If you're in a hurry
  hopa!("No ni mă la el"); // In SFW contexts
}
```

## Other examples

See the [examples](examples/src/main.rs) to get a rough sense of the whole
syntax. It's that _ușor_.

## _Contribuții_

First of all, _mulțumesc frumos_ for considering participating to this joke, the
Romanian government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `principal` (Romanian for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Romanian.

## But why would you do _așa ceva_?

* Playing around with proc macros
* _Îi fain_

## Other languages

Here's a non-exhaustive list of implementations for other languages:

* French: [rouille](https://github.com/bnjbvr/rouille)
* Dutch: [roest](https://github.com/jeroenhd/roest)
* German: [rost](https://github.com/michidk/rost)
* Polish: [rdza](https://github.com/phaux/rdza)
* Italian: [ruggine](https://github.com/DamianX/ruggine)
* Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
* Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
* Hindi: [zung](https://github.com/rishit-khandelwal/zung)
* Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
* Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
* Spanish: [oxido](https://github.com/fdschonborn/oxido)
* Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
* Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
* Arabic: [sada](https://github.com/LAYGATOR/sada)
* Turkish: [pas](https://github.com/ekimb/pas)
* Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
* Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
* Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
* Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)

You can find a more comprehensive list on [@bnjbvr](https://github.com/bnjbvr)'s [rouille](https://github.com/bnjbvr/rouille#other-languages) repo.

## _Mulțumiri_

* [@bnjbvr](https://github.com/bnjbvr) for the idea, as well as the base repo

## Licență

[WTFPL](http://www.wtfpl.net/)
