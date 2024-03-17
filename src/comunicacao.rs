use std::collections::HashMap;

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
    mensagens_do_controlador: HashMap<String, MensagemDoControlador>,
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
        self.mensagens_do_controlador.remove(placa)
    }
    pub fn send_por_controlador(&mut self, placa: String, message: MensagemDoControlador) {
        self.mensagens_do_controlador.insert(placa, message);
    }
    pub fn receive_por_controlador(&mut self) -> Option<MensagemDeVeiculo> {
        if self.mensagens_de_veiculo.len() == 0 {
            return Option::None;
        } else {
            return Option::Some(self.mensagens_de_veiculo.swap_remove(0));
        }
    }
}
