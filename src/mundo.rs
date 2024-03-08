pub mod veiculo;

use veiculo::Carro;

use crate::{VIAH_LARGURA, VIAH_MARGEM, VIAV_LARGURA, VIAV_MARGEM, VIA_MAXIMO_CARROS};
pub struct Transito {
    pub num_carros_criados_h: usize,
    pub num_carros_sairam_h: usize,
    pub carros_via_h: [Carro; 4],
    pub num_carros_criados_v: usize,
    pub num_carros_sairam_v: usize,
    pub carros_via_v: [Carro; 4],
}

impl Transito {
    pub fn new() -> Self {
        Self {
            num_carros_criados_h: 0,
            num_carros_sairam_h: 0,
            carros_via_h: [
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
            ],
            num_carros_criados_v: 0,
            num_carros_sairam_v: 0,
            carros_via_v: [
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
            ],
        }
    }
    pub fn ocorreu_colisao(&self) -> Option<&str> {
        let mut i = self.num_carros_sairam_h + 1;
        while i < self.num_carros_sairam_h {
            if self.carros_via_h[i - 1].pos_atual - self.carros_via_h[i - 1].comprimento
                <= self.carros_via_h[i].pos_atual
            {
                return Some("Colisão via H, carros {}");
            }
            i += 1;
        }
        i = self.num_carros_sairam_v + 1;
        while i < self.num_carros_sairam_v {
            if self.carros_via_v[i - 1].pos_atual - self.carros_via_v[i - 1].comprimento
                <= self.carros_via_v[i].pos_atual
            {
                return Some("Colisão via V, carros {}");
            }
            i += 1;
        }

        let mut cruzando_h = false;
        let mut cruzando_v = false;
        i = self.num_carros_sairam_h;
        while i < self.num_carros_sairam_h {
            cruzando_h = cruzando_h
                || (self.carros_via_h[i].pos_atual > 0.0
                    && self.carros_via_h[i].pos_atual
                        < 0.0 + VIAV_LARGURA + self.carros_via_h[i].comprimento);
            i += 1;
        }
        i = self.num_carros_sairam_v;
        while i < self.num_carros_sairam_v {
            cruzando_v = cruzando_v
                || (self.carros_via_v[i].pos_atual > 0.0
                    && self.carros_via_v[i].pos_atual
                        < 0.0 + VIAH_LARGURA + self.carros_via_v[i].comprimento);
            i += 1;
        }
        if cruzando_h && cruzando_v {
            return Some("Colisão dentreo do cruzamento");
        }

        None
    }

    pub fn chega_carro(&mut self, via: Via, acel: f64) -> bool {
        let jah_tem = match via {
            Via::ViaH => self.num_carros_criados_h,
            Via::VIaV => self.num_carros_criados_v,
        };
        if jah_tem == VIA_MAXIMO_CARROS {
            return false;
        }
        let mut nova_placa = String::from("CCC");
        nova_placa.push_str(&format!("{:04}", jah_tem + 1));
        let novo_carro = Carro::new(nova_placa, via.clone(), acel);
        match via {
            Via::ViaH => {
                self.carros_via_h[self.num_carros_criados_h] = novo_carro;
                self.num_carros_criados_h += 1;
            }
            Via::VIaV => {
                self.carros_via_v[self.num_carros_criados_v] = novo_carro;
                self.num_carros_criados_v += 1;
            }
        }
        true
    }

    pub fn tick(&mut self, tickms: f64) {
        println!("transito.tick");
        let mut i;
        i = self.num_carros_sairam_h;
        while i < self.num_carros_sairam_h {
            self.carros_via_h[i].tick(tickms);
            i += 1;
        }
        i = self.num_carros_sairam_v;
        while i < self.num_carros_sairam_v {
            self.carros_via_v[i].tick(tickms);
            i += 1;
        }

        // Carro mais antigo na via H saiu do sistema?
        if self.num_carros_sairam_h < self.num_carros_criados_h {
            let mais_antigo_h = &self.carros_via_h[self.num_carros_sairam_h];
            if mais_antigo_h.pos_atual > mais_antigo_h.comprimento + VIAV_LARGURA + VIAH_MARGEM {
                println!("@{} saiu da via H", mais_antigo_h.placa);
                self.num_carros_sairam_h += 1;
            }
        }

        if self.num_carros_sairam_v < self.num_carros_criados_v {
            let mais_antigo_v = &self.carros_via_v[self.num_carros_sairam_v];
            if mais_antigo_v.pos_atual > mais_antigo_v.comprimento + VIAH_LARGURA + VIAV_MARGEM {
                println!("@{} saiu da via V", mais_antigo_v.placa);
                self.num_carros_sairam_v += 1;
            }
        }
    }

    pub fn mostra_vias(&self) {
        println!("___Carros na via H___");
        let mut i = self.num_carros_sairam_h;
        while i < self.num_carros_criados_h {
            self.carros_via_h[i].mostra();
            i += 1;
        }
        println!("___Carros na via V___");
        i = self.num_carros_sairam_v;
        while i < self.num_carros_criados_v {
            self.carros_via_v[i].mostra();
            i += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub enum Via {
    ViaH,
    VIaV,
}
