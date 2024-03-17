use std::collections::HashMap;

use crate::{
    comunicacao::{Comunicacao, MensagemDeVeiculo, MensagemDoControlador},
    mundo::Via,
};

struct Situacao {
    placa: String,
    via: Via,
    acel_max: f64,
    acel_min: f64,
    vel_max: f64,
    comprimento: f64,
    pos_atual: f64,
    vel_atual: f64,
    acel_atual: f64,
}

pub struct Controlador {
    situacao: HashMap<String, Situacao>,
}

impl Controlador {
    pub fn new() -> Self {
        Self {
            situacao: HashMap::new(),
        }
    }
    pub fn controle(&mut self, comunicacao: &mut Comunicacao) {
        loop {
            match comunicacao.receive_por_controlador() {
                None => break,
                Some(message) => match message {
                    MensagemDeVeiculo::Chegada {
                        placa,
                        via,
                        acel_max,
                        acel_min,
                        vel_max,
                        comprimento,
                    } => {
                        let novo = Situacao {
                            placa: placa.clone(),
                            via,
                            acel_max,
                            acel_min,
                            vel_max,
                            comprimento,
                            pos_atual: 0.0,
                            vel_atual: 0.0,
                            acel_atual: 0.0,
                        };
                        self.situacao.insert(novo.placa.clone(), novo);
                    }
                    MensagemDeVeiculo::SituacaoAtual {
                        placa,
                        pos_atual,
                        vel_atual,
                        acel_atual,
                    } => {
                        let velho = self.situacao.get_mut(&placa);
                        match velho {
                            None => (),
                            Some(veiculo) => {
                                veiculo.pos_atual = pos_atual;
                                veiculo.vel_atual = vel_atual;
                                veiculo.acel_atual = acel_atual;
                            }
                        }
                    }
                },
            }
        }
        for (placa, situacao) in &self.situacao {
            println!("#controlador solicita situacao de @{placa}");
            let message = MensagemDoControlador::PedeSituacao {
                placa: placa.to_string(),
            };
            comunicacao.send_por_controlador(placa.to_string(), message);
        }
    }
}
