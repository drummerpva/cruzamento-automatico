use std::{thread::sleep, time::Duration};

mod comunicacao;
mod controlador;
mod mundo;
use mundo::{Transito, Via};

fn simula_mundo() {
    const TICKMS: f64 = 100.0;
    const TEMPO_ENTRE_CHEGADAS: f64 = 300.0;
    const TEMPO_ENTRE_CONTROLES: f64 = 1000.0;
    let mut comunicacao = Comunicacao::new();
    let mut transito = Transito::new();
    transito.chega_carro(Via::ViaH, &mut comunicacao);
    transito.chega_carro(Via::VIaV, &mut comunicacao);
    let mut tempo_ate_proxima_chegada = TEMPO_ENTRE_CHEGADAS;
    let mut controlador = Controlador::new();
    let mut tempo_ate_proximo_controle = TEMPO_ENTRE_CONTROLES;
    println!("simula carros");
    loop {
        sleep(Duration::from_millis(TICKMS.round() as u64));
        transito.tick(tickms, &mut comunicacao);
        transito.mostra_vias();
        match transito.ocorreu_colisao() {
            Some(mensagem) => {
                panic!("Ocorreu colisão: {}", mensagem);
            }
            None => {}
        }
        if transito.vazio() {
            break;
        }
        tempo_ate_proxima_chegada -= TICKMS;
        if tempo_ate_proxima_chegada <= 0.0 {
            assert!(
                transito.chega_carro(Via::ViaH, &mut comunicacao),
                "Falha ao chegar um carro na via H"
            );
            assert!(
                transito.chega_carro(Via::VIaV, &mut comunicacao),
                "Falha ao chegar um carro na via V"
            );
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
