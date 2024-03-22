use std::collections::{HashMap, VecDeque};

use crate::mundo::Via;

pub enum MensagemDeVeiculo {
    Chegada {
        placa: String,
        via: Via,
        acel_max: f64,
        acel_min: f64,
        vel_max: f64,
        comprimento: f64,
    },
    SituacaoAtual {
        placa: String,
        pos_atual: f64,
        vel_atual: f64,
        acel_atual: f64,
    },
}
pub enum MensagemDoControlador {
    SetAcel { placa: String, acel: f64 },
    PedeSituacao { placa: String },
}
pub struct Comunicacao {
    mensagens_de_veiculo: Vec<MensagemDeVeiculo>,
    mensagens_do_controlador: HashMap<String, VecDeque<MensagemDoControlador>>,
}
impl Comunicacao {
    pub fn new() -> Self {
        Self {
            mensagens_de_veiculo: Vec::new(),
            mensagens_do_controlador: HashMap::new(),
        }
    }
    pub fn send_por_veiculo(&mut self, message: MensagemDeVeiculo) {
        self.mensagens_de_veiculo.push(message);
    }
    pub fn receive_por_veiculo(&mut self, placa: &String) -> Option<MensagemDoControlador> {
        match self.mensagens_do_controlador.get_mut(placa) {
            None => None,
            Some(messages) => messages.pop_front(),
        }
    }
    pub fn send_por_controlador(&mut self, placa: String, message: MensagemDoControlador) {
        let lista = self
            .mensagens_do_controlador
            .entry(placa)
            .or_insert(VecDeque::new());
        lista.push_back(message);
    }
    pub fn receive_por_controlador(&mut self) -> Option<MensagemDeVeiculo> {
        if self.mensagens_de_veiculo.len() == 0 {
            return Option::None;
        } else {
            return Option::Some(self.mensagens_de_veiculo.swap_remove(0));
        }
    }
}
