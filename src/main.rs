use std::{thread::sleep, time::Duration};

mod comunicacao;
mod controlador;
mod mundo;
use mundo::{Transito, Via};

use crate::{comunicacao::Comunicacao, controlador::Controlador};

fn simula_mundo() {
    const TICKMS: f64 = 100.0;
    const TEMPO_ENTRE_CHEGADAS: f64 = 300.0;
    const TEMPO_ENTRE_CONTROLES: f64 = 1000.0;
    let mut comunicacao = Comunicacao::new();
    let mut transito = Transito::new();
    if let Err(message) = transito.chega_carro(Via::ViaH, &mut comunicacao) {
        println!("Falha em chegar o primeiro carro na via H: {}", message);
    }
    if let Err(message) = transito.chega_carro(Via::VIaV, &mut comunicacao) {
        println!("Falha em chegar o primeiro carro na via V: {}", message);
    }
    let mut tempo_ate_proxima_chegada = TEMPO_ENTRE_CHEGADAS;
    let mut controlador = Controlador::new();
    let mut tempo_ate_proximo_controle = TEMPO_ENTRE_CONTROLES;
    println!("simula carros");
    loop {
        sleep(Duration::from_millis(TICKMS.round() as u64));
        transito.tick(TICKMS, &mut comunicacao);
        transito.mostra_vias();
        match transito.ocorreu_colisao() {
            Some(mensagem) => {
                println!("Ocorreu Colisão: {}", mensagem);
                return;
            }
            None => {}
        }
        if transito.vazio() {
            println!("Nenhum carro no perimetro");
            break;
        }
        tempo_ate_proxima_chegada -= TICKMS;
        if tempo_ate_proxima_chegada <= 0.0 {
            match transito.chega_carro(Via::ViaH, &mut comunicacao) {
                Ok(_) => (),
                Err(message) => {
                    println!("Falha ao chegar um carro na via H: {}", message);
                }
            }
            match transito.chega_carro(Via::VIaV, &mut comunicacao) {
                Ok(_) => (),
                Err(message) => {
                    println!("Falha ao chegar um carro na via V: {}", message);
                }
            }
            tempo_ate_proxima_chegada += TEMPO_ENTRE_CHEGADAS;
        }
        tempo_ate_proximo_controle -= TICKMS;
        if tempo_ate_proximo_controle <= 0.0 {
            controlador.controle(&mut comunicacao);
            tempo_ate_proximo_controle += TEMPO_ENTRE_CONTROLES;
        }
    }
}

fn main() {
    println!("Inicio do programa");
    simula_mundo();
    println!("Fim da simulação");
}
