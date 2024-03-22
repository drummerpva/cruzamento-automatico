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
    acel_desejada: f64,
}
struct Semaforo {
    tempo_verde: f64,
    tempo_amarelo: f64,
    restam_verde: f64,
    restam_amarelo: f64,
    via_verde: Via,
    via_vermelho: Via,
    amarelo: bool,
    display_tudo: bool,
}
impl Semaforo {
    fn new(display_tudo: bool) -> Self {
        Self {
            tempo_verde: 13.0,
            tempo_amarelo: 5.0,
            restam_verde: 13.0,
            restam_amarelo: 5.0,
            via_verde: Via::ViaH,
            via_vermelho: Via::ViaV,
            amarelo: false,
            display_tudo,
        }
    }
    fn estrategia(&mut self, ms_decorrido: f64, situacao: &mut HashMap<String, Situacao>) {
        let tempo_decorrido = ms_decorrido / 1000.0;
        if self.amarelo {
            self.restam_amarelo -= tempo_decorrido;
            if self.restam_amarelo > 0.0 {
                // Continua como estava
            } else {
                self.amarelo = false;
                if self.via_verde == Via::ViaH {
                    self.via_verde = Via::ViaV;
                    self.via_vermelho = Via::ViaH;
                } else {
                    self.via_verde = Via::ViaH;
                    self.via_vermelho = Via::ViaV;
                }
                self.restam_amarelo = 0.0;
                self.restam_verde = self.tempo_verde;
            }
        } else {
            self.restam_verde -= tempo_decorrido;
            if self.restam_verde > 0.0 {
                // Continua como estava
            } else {
                self.amarelo = true;
                self.restam_verde = 0.0;
                self.restam_amarelo = self.tempo_amarelo;
            }
        }
        if self.display_tudo {
            println!(
                "nVerde {:?} {}    nVermelho {:?}   restam_amarelo {}",
                self.via_verde, self.restam_verde, self.via_vermelho, self.restam_amarelo
            );
        }

        #[derive(Debug)]
        struct MiniSituacao {
            placa: String,
            pos_atual: f64,
        }
        let mut ordem_via_h: Vec<MiniSituacao> = Vec::new();
        let mut ordem_via_v: Vec<MiniSituacao> = Vec::new();
        let situacao_iter = situacao.iter();
        for (k, v) in situacao_iter {
            if v.via == Via::ViaH {
                ordem_via_h.push(MiniSituacao {
                    placa: v.placa.clone(),
                    pos_atual: v.pos_atual,
                });
            } else {
                ordem_via_v.push(MiniSituacao {
                    placa: v.placa.clone(),
                    pos_atual: v.pos_atual,
                });
            }
        }

        ordem_via_h.sort_unstable_by(|a, b| b.pos_atual.partial_cmp(&a.pos_atual).unwrap());
        ordem_via_v.sort_unstable_by(|a, b| b.pos_atual.partial_cmp(&a.pos_atual).unwrap());
        //println!("{:?}", ordem_via_h);

        let ordem_via_vermelho: Vec<MiniSituacao>;
        let ordem_via_verde: Vec<MiniSituacao>;
        if self.via_vermelho == Via::ViaH {
            ordem_via_vermelho = ordem_via_h;
            ordem_via_verde = ordem_via_v;
        } else {
            ordem_via_vermelho = ordem_via_v;
            ordem_via_verde = ordem_via_h;
        }

        let espacamento = 4.0;
        let mut pos_alvo = -espacamento;

        for mini in ordem_via_vermelho {
            let veiculo = situacao.get_mut(&mini.placa).unwrap();
            if veiculo.pos_atual >= pos_alvo {
                veiculo.acel_desejada = veiculo.acel_min;
            } else {
                veiculo.acel_desejada =
                    veiculo.vel_atual.powi(2) / (2.0 * (pos_alvo - veiculo.pos_atual));
                if veiculo.acel_desejada < veiculo.acel_min {
                    veiculo.acel_desejada = veiculo.acel_min;
                }
            }
            pos_alvo -= veiculo.comprimento + espacamento;
        }
    }
}

pub struct Controlador {
    situacao: HashMap<String, Situacao>,
    semaforo: Semaforo,
    display_tudo: bool,
}

impl Controlador {
    pub fn new() -> Self {
        Self {
            situacao: HashMap::new(),
            semaforo: Semaforo::new(true),
            display_tudo: true,
        }
    }
    pub fn controle(&mut self, tempo_decorrido: f64, comunicacao: &mut Comunicacao) {
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
                            acel_desejada: 0.0,
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
        self.semaforo
            .estrategia(tempo_decorrido, &mut self.situacao);

        for (k, v) in &self.situacao {
            let message = MensagemDoControlador::SetAcel {
                placa: k.to_string(),
                acel: v.acel_desejada,
            };
            comunicacao.send_por_controlador(k.to_string(), message);
            if self.display_tudo {
                println!(
                    "#controlador setAceleracao de @{} em {}",
                    k.to_string(),
                    v.acel_desejada
                );
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
