use super::Via;
use crate::comunicacao::{Comunicacao, MensagemDeVeiculo, MensagemDoControlador};

use crate::mundo::{VIAH_PERIMETRO, VIAV_PERIMETRO};

pub const _CARRO_LARGURA: f64 = 2.0;
pub const CARRO_COMPRIMENTO: f64 = 4.0;
pub const VELOCIDADE_CRUZEIRO: f64 = 80.0 * (1000.0 / 3600.0);
pub const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);
pub const ACELERACAO_MAXIMA: f64 = 3.0;
pub const ACELERACAO_MINIMA: f64 = -10.0;

pub struct Carro {
    pub placa: String,
    pub via: Via,
    pub acel_max: f64,
    pub acel_min: f64,
    pub vel_max: f64,
    pub comprimento: f64,
    pub pos_atual: f64,
    pub vel_atual: f64,
    pub acel_atual: f64,
}

impl Carro {
    pub fn new(placa: String, via: Via, acel: f64) -> Self {
        let (placa_valida, message) = Carro::valida_placa(&placa);
        assert!(placa_valida, " Placa inválida: {message} @{placa}");
        assert!(
            acel >= ACELERACAO_MINIMA && acel <= ACELERACAO_MAXIMA,
            " Aceleração inválida: {placa} {acel}"
        );
        Self {
            placa,
            via: via.clone(),
            acel_max: ACELERACAO_MAXIMA,
            acel_min: ACELERACAO_MINIMA,
            vel_max: VELOCIDADE_MAXIMA,
            comprimento: CARRO_COMPRIMENTO,
            pos_atual: match via {
                Via::ViaH => -VIAH_PERIMETRO,
                Via::ViaV => -VIAV_PERIMETRO,
            },
            vel_atual: VELOCIDADE_CRUZEIRO,
            acel_atual: acel,
        }
    }
    pub fn valida_placa(placa: &str) -> (bool, &str) {
        if !placa.is_ascii() {
            return (false, "Placa não é ASCII");
        }
        if placa.len() != 7 {
            return (false, "Placa não tem 7 caracteres");
        }
        if !placa[0..3].chars().all(char::is_alphabetic) {
            return (false, "Placa não tem 3 letras iniciais");
        }
        if !placa[3..].chars().all(char::is_numeric) {
            return (false, "Placa não tem 4 dígitos finais");
        }
        (true, "")
    }
    pub fn mostra(&self) {
        println!(
            "@{}, na posicão {:?}{}, velocidade {}, aceleração {}",
            self.placa, self.via, self.pos_atual, self.vel_atual, self.acel_atual
        );
    }
    pub fn tick(&mut self, tickms: f64, comunicacao: &mut Comunicacao) {
        let pos_anterior = self.pos_atual;
        self.pos_atual = self.pos_atual
            + self.vel_atual * (tickms / 1000.0)
            + self.acel_atual * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;
        self.vel_atual = self.vel_atual + self.acel_atual * (tickms / 1000.0);

        if self.pos_atual < pos_anterior {
            self.pos_atual = pos_anterior;
        }
        if self.vel_atual < 0.0 {
            self.vel_atual = 0.0;
        }
        if self.vel_atual > self.vel_max {
            self.vel_atual = self.vel_max;
        }
        loop {
            match comunicacao.receive_por_veiculo(&self.placa) {
                None => break,
                Some(message) => match message {
                    MensagemDoControlador::SetAcel { placa, acel } => {
                        assert!(&placa == &self.placa);
                        println!("#veiculo @{} recebe acel {}", placa, acel);
                        self.acel_atual = acel;
                    }
                    MensagemDoControlador::PedeSituacao { placa } => {
                        println!("#veiculo @{} informa sua situacao", &self.placa);
                        let message = MensagemDeVeiculo::SituacaoAtual {
                            placa,
                            pos_atual: self.pos_atual,
                            vel_atual: self.vel_atual,
                            acel_atual: self.acel_atual,
                        };
                        comunicacao.send_por_veiculo(message);
                    }
                },
            }
        }
    }
}
