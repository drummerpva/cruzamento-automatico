use std::{thread::sleep, time::Duration};

mod mundo;
use mundo::{Transito, Via};

const VIAH_MARGEM: f64 = 15.0;
const VIAV_MARGEM: f64 = 15.0;

const VIAH_LARGURA: f64 = 4.0;
const VIAV_LARGURA: f64 = 4.0;

const VIAH_PERIMETRO: f64 = 150.0;
const VIAV_PERIMETRO: f64 = 150.0;

const _CARRO_LARGURA: f64 = 2.0;
const CARRO_COMPRIMENTO: f64 = 4.0;

const VIA_MAXIMO_CARROS: usize = 4;

const VELOCIDADE_CRUZEIRO: f64 = 80.0 * (1000.0 / 3600.0);
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);
const ACELERACAO_MAXIMA: f64 = 3.0;
const ACELERACAO_MINIMA: f64 = -10.0;

fn simula_carros() {
    const TEMPO_ENTRE_CHEGADAS: f64 = 300.0;
    let mut transito = Transito::new();
    transito.chega_carro(Via::ViaH, ACELERACAO_MAXIMA);
    transito.chega_carro(Via::VIaV, ACELERACAO_MAXIMA);
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

        if transito.num_carros_criados_h == transito.num_carros_sairam_h
            && transito.num_carros_criados_v == transito.num_carros_sairam_v
        {
            break;
        }

        tempo_ate_proxima_chegada -= tickms;

        if tempo_ate_proxima_chegada <= 0.0 {
            let _acel = 0.0;
            assert!(
                transito.chega_carro(Via::ViaH, ACELERACAO_MAXIMA),
                "Falha ao chegar um carro na via H"
            );
            assert!(
                transito.chega_carro(Via::VIaV, ACELERACAO_MAXIMA),
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
