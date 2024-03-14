use std::{thread::sleep, time::Duration};

mod mundo;
use mundo::{Transito, Via};

fn simula_carros() {
    const TEMPO_ENTRE_CHEGADAS: f64 = 300.0;
    let mut transito = Transito::new();
    transito.chega_carro(Via::ViaH);
    transito.chega_carro(Via::VIaV);
    let mut tempo_ate_proxima_chegada = TEMPO_ENTRE_CHEGADAS;
    println!("simula carros");
    let mut tickms: f64;
    loop {
        tickms = 100.0;
        sleep(Duration::from_millis(tickms.round() as u64));
        transito.tick(tickms);
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

        tempo_ate_proxima_chegada -= tickms;

        if tempo_ate_proxima_chegada <= 0.0 {
            assert!(
                transito.chega_carro(Via::ViaH),
                "Falha ao chegar um carro na via H"
            );
            assert!(
                transito.chega_carro(Via::VIaV),
                "Falha ao chegar um carro na via V"
            );
            tempo_ate_proxima_chegada += TEMPO_ENTRE_CHEGADAS;
        }
    }
}

fn main() {
    println!("Inicio do programa");
    simula_carros();
    println!("Fim da simulação");
}
