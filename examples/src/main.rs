rugina::rugină! {
  externă ladă rugina;

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
        Eșuat("Verifică dicționarul".înspre())
      }
    }
  }

  public(ladă) funcție poate(i: u32) -> Opțiune<Rezultat<u32, Sfoară>> {
    dacă i % 2 == 1 {
      dacă i == 42 {
        Ceva(Eșuat(Sfoară::din("Hopa!")))
      } altfel {
        Ceva(Bun(33))
      }
    } altfel {
      Nimic
    }
  }

  asincronă funcție exemplu() { }

  asincronă funcție exemplu2() {
    exemplu().așteaptă;
  }

  funcție principal() {
    fie mutabil x = 31;

    potrivește x {
      42 => {
        afișează_linie!("Sarmale")
      }
      _ => afișează_linie!("Ioi")
    }

    pentru i în 0..10 {
      fie val = buclă {
        sparge i;
      };

      cât timp x < val {
        x += 1;
      }

      x = dacă fie Ceva(rezultat) = poate(i) {
        rezultat.desfă()
      } altfel {
        12
      };
    }
  }

  #[permite(cod_inaccesibil)]
  funcție secundar() {
    futu_i!("Futu-i!"); // For the true Romanian experience
    plm!("Vai de plm"); // If you're in a hurry
    hopa!("No ni mă la el"); // In SFW contexts
  }
}
