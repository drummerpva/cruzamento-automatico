mod veiculo;
use self::veiculo::VELOCIDADE_CRUZEIRO;
use crate::comunicacao::Comunicacao;
use crate::comunicacao::MensagemDeVeiculo;
use veiculo::Carro;

const VIAH_MARGEM: f64 = 15.0;
const VIAV_MARGEM: f64 = 15.0;

const VIAH_LARGURA: f64 = 4.0;
const VIAV_LARGURA: f64 = 4.0;

const VIAH_PERIMETRO: f64 = 150.0;
const VIAV_PERIMETRO: f64 = 150.0;

const VIA_MAXIMO_CARROS: usize = 4;

#[derive(Debug, Clone, PartialEq)]
pub enum Via {
    ViaH,
    ViaV,
}

pub struct Transito {
    carros_via_h: Vec<Carro>,
    carros_via_v: Vec<Carro>,
    carros_criados: i32,
}

impl Transito {
    pub fn new() -> Self {
        Self {
            carros_via_h: Vec::new(),
            carros_via_v: Vec::new(),
            carros_criados: 0,
        }
    }
    pub fn ocorreu_colisao(&self) -> Option<&str> {
        if self.carros_via_h.len() >= 2 {
            for i in 0..self.carros_via_h.len() - 1 {
                let traseira_do_i = self.carros_via_h.get(i).unwrap().pos_atual
                    - self.carros_via_h.get(i).unwrap().comprimento;
                if traseira_do_i <= self.carros_via_h.get(i + 1).unwrap().pos_atual {
                    return Some("Colisão via H");
                }
            }
        }
        if self.carros_via_v.len() >= 2 {
            for i in 0..self.carros_via_v.len() - 1 {
                let traseira_do_i = self.carros_via_v.get(i).unwrap().pos_atual
                    - self.carros_via_v.get(i).unwrap().comprimento;
                if traseira_do_i <= self.carros_via_v.get(i + 1).unwrap().pos_atual {
                    return Some("Colisão via V");
                }
            }
        }
        let mut cruzando_h = false;
        let mut cruzando_v = false;
        for carro in &self.carros_via_h {
            cruzando_h = cruzando_h
                || (carro.pos_atual > 0.0
                    && carro.pos_atual < 0.0 + VIAV_LARGURA + carro.comprimento);
        }
        for carro in &self.carros_via_v {
            cruzando_v = cruzando_v
                || (carro.pos_atual > 0.0
                    && carro.pos_atual < 0.0 + VIAH_LARGURA + carro.comprimento);
        }
        if cruzando_h && cruzando_v {
            return Some("Colisão dentro do cruzamento");
        }
        None
    }

    fn define_velocidade_chegada(&self, via: &Via) -> f64 {
        match via {
            Via::ViaH => {
                if self.carros_via_h.len() == 0 {
                    return VELOCIDADE_CRUZEIRO;
                }
                let ultimo_carro = self.carros_via_h.last().unwrap();
                let distancia = VIAH_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento;
                if distancia < 0.5 {
                    return 0.0;
                }
                if distancia < 4.0 {
                    return VELOCIDADE_CRUZEIRO.min(ultimo_carro.vel_atual);
                }
                return VELOCIDADE_CRUZEIRO;
            }
            Via::ViaV => {
                if self.carros_via_v.len() == 0 {
                    return VELOCIDADE_CRUZEIRO;
                }
                let ultimo_carro = self.carros_via_v.last().unwrap();
                let distancia = VIAV_PERIMETRO + ultimo_carro.pos_atual - ultimo_carro.comprimento;
                if distancia < 0.5 {
                    return 0.0;
                }
                if distancia < 4.0 {
                    return VELOCIDADE_CRUZEIRO.min(ultimo_carro.vel_atual);
                }
                return VELOCIDADE_CRUZEIRO;
            }
        }
    }

    pub fn chega_carro(&mut self, via: Via, comunicacao: &mut Comunicacao) -> Result<(), String> {
        let velocidade = self.define_velocidade_chegada(&via);
        if velocidade == 0.0 {
            return Err("Via congetionada".to_string());
        }
        let mut nova_placa = String::from("CCC");
        nova_placa.push_str(&format!("{:04}", self.carros_criados));
        self.carros_criados += 1;
        let novo_carro = Carro::new(nova_placa.clone(), via.clone(), 0.0);
        comunicacao.send_por_veiculo(MensagemDeVeiculo::Chegada {
            placa: nova_placa,
            via: via.clone(),
            acel_max: novo_carro.acel_max,
            acel_min: novo_carro.acel_min,
            vel_max: novo_carro.vel_max,
            comprimento: novo_carro.comprimento,
        });
        match via {
            Via::ViaH => {
                self.carros_via_h.push(novo_carro);
            }
            Via::ViaV => {
                self.carros_via_v.push(novo_carro);
            }
        }
        Ok(())
    }
    pub fn vazio(&self) -> bool {
        self.carros_via_h.len() == 0 && self.carros_via_v.len() == 0
    }

    pub fn tick(&mut self, tickms: f64, comunicacao: &mut Comunicacao) {
        println!("transito.tick");
        for carro in &mut self.carros_via_h {
            carro.tick(tickms, comunicacao);
        }
        for carro in &mut self.carros_via_v {
            carro.tick(tickms, comunicacao);
        }

        // Carro mais antigo na via H saiu do sistema?
        if self.carros_via_h.len() > 0 {
            let mais_antigo_h = self.carros_via_h.get(0).unwrap();
            if mais_antigo_h.pos_atual > mais_antigo_h.comprimento + VIAV_LARGURA {
                println!("@{} saiu da via H", mais_antigo_h.placa);
                self.carros_via_h.remove(0);
            }
        }

        // Carro mais antigo na via V saiu do sistema?
        if self.carros_via_v.len() > 0 {
            let mais_antigo_v = self.carros_via_v.get(0).unwrap();
            if mais_antigo_v.pos_atual > mais_antigo_v.comprimento + VIAH_LARGURA {
                println!("@{} saiu da via V", mais_antigo_v.placa);
                self.carros_via_v.remove(0);
            }
        }
    }

    pub fn mostra_vias(&self) {
        println!("___Carros na via H___");
        for carro in &self.carros_via_h {
            carro.mostra();
        }
        println!("___Carros na via V___");
        for carro in &self.carros_via_v {
            carro.mostra();
        }
    }
}
