use std::{thread::sleep, time::Duration};

const _VIAH_MARGEM: f64 = 15.0;
const _VIAV_MARGEM: f64 = 15.0;

const VIAH_LARGURA: f64 = 4.0;
const VIAV_LARGURA: f64 = 4.0;

const _VIAH_PERIMETRO: f64 = 150.0;
const _VIAV_PERIMETRO: f64 = 150.0;

const _CARRO_LARGURA: f64 = 2.0;
const CARRO_COMPRIMENTO: f64 = 4.0;

const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);
const ACELERACAO_MAXIMA: f64 = 3.0;
const ACELERACAO_MINIMA: f64 = -10.0;

fn simula_carros(via_carro1: char, acel_carro1: f64, via_carro2: char, acel_carro2: f64) -> bool {
    let mut placa1 = String::from("ABC1234");
    let via1 = via_carro1;
    let _acel_max1 = ACELERACAO_MAXIMA;
    let _acel_min1 = ACELERACAO_MINIMA;
    let vel_max1 = VELOCIDADE_MAXIMA;
    let comprimento1 = CARRO_COMPRIMENTO;
    let mut pos_atual1 = -80.0;
    let mut vel_atual1 = 0.0;
    let acel_atual1: f64;

    let mut placa2 = String::from("xyz9876");
    let via2 = via_carro2;
    let _acel_max2 = ACELERACAO_MAXIMA;
    let _acel_min2 = ACELERACAO_MINIMA;
    let vel_max2 = VELOCIDADE_MAXIMA;
    let comprimento2 = CARRO_COMPRIMENTO;
    let mut pos_atual2 = -100.0;
    let mut vel_atual2 = 0.0;
    let acel_atual2: f64;

    placa1 = placa1.to_uppercase();
    placa2 = placa2.to_uppercase();
    if !valida_placa(&placa1) {
        panic!("    Placa inválida: {placa1}");
    }
    if !valida_placa(&placa2) {
        panic!("    Placa inválida: {placa2}");
    }

    acel_atual1 = acel_carro1;
    acel_atual2 = acel_carro2;

    println!("Início da simulação");
    let mut tickms: f64;

    loop {
        sleep(Duration::from_millis(100));
        tickms = 100.0;

        // Atualiza carro 1
        let old_position = pos_atual1;

        pos_atual1 = pos_atual1
            + vel_atual1 * (tickms / 1000.0)
            + acel_atual1 * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;
        vel_atual1 = vel_atual1 + acel_atual1 * (tickms / 1000.0);

        if pos_atual1 < old_position {
            pos_atual1 = old_position;
        }
        if vel_atual1 < 0.0 {
            vel_atual1 = 0.0;
        }
        if vel_atual1 > vel_max1 {
            vel_atual1 = vel_max1;
        }

        println!("Carro1 {placa1} na posição {via1}{pos_atual1}, velocidade {vel_atual1}, aceleração {acel_atual1}");

        // Atualiza carro 2
        let old_position = pos_atual2;

        pos_atual2 = pos_atual2
            + vel_atual2 * (tickms / 1000.0)
            + acel_atual2 * (tickms / 1000.0) * (tickms / 1000.0) / 2.0;
        vel_atual2 = vel_atual2 + acel_atual2 * (tickms / 1000.0);

        if pos_atual2 < old_position {
            pos_atual2 = old_position;
        }
        if vel_atual2 < 0.0 {
            vel_atual2 = 0.0;
        }
        if vel_atual2 > vel_max2 {
            vel_atual2 = vel_max2;
        }

        println!("Carro2 {placa2} na posição {via2}{pos_atual2}, velocidade {vel_atual2}, aceleração {acel_atual2}");

        // Detecta colisão na via H
        if via1 == 'H' && via2 == 'H' {
            if colisao_longitudinal(pos_atual1, comprimento1, pos_atual2) {
                println!("Colisão na via H");
                return true;
            }
        }

        // Detecta colisão na via V
        if via1 == 'V' && via2 == 'V' {
            if colisao_longitudinal(pos_atual1, comprimento1, pos_atual2) {
                println!("Colisão na via V");
                return true;
            }
        }

        // Detecta colisão no cruzamento
        if via1 != via2 {
            if dentro_cruzamento(pos_atual1, comprimento1, via1)
                && dentro_cruzamento(pos_atual2, comprimento2, via2)
            {
                println!("Colisão no cruzamento");
                return true;
            }
        }

        // Verifica se carro 1 saiu do sistema
        if pos_atual1
            > comprimento1
                + if via1 == 'H' {
                    VIAV_LARGURA
                } else {
                    VIAH_LARGURA
                }
        {
            break;
        }

        // verifica se carro 2 saiu do sistema
        if pos_atual2
            > comprimento2
                + if via2 == 'H' {
                    VIAV_LARGURA
                } else {
                    VIAH_LARGURA
                }
        {
            break;
        }
    }
    return false;
}

fn valida_placa(placa: &str) -> bool {
    if !placa.is_ascii() {
        println!("Placa não é ASCII");
        return false;
    }
    if placa.len() != 7 {
        println!("Placa não tem 7 caracteres");
        return false;
    }
    if !placa[0..3].chars().all(char::is_alphabetic) {
        println!("Placa não tem 3 letras iniciais");
        return false;
    }
    if !placa[3..].chars().all(char::is_numeric) {
        println!("Placa não tem 4 dígitos finais");
        return false;
    }
    return true;
}

fn colisao_longitudinal(posicao_frente: f64, comprimento: f64, posicao_tras: f64) -> bool {
    return posicao_frente - comprimento <= posicao_tras;
}

fn dentro_cruzamento(posicao: f64, comprimento: f64, via: char) -> bool {
    return posicao > 0.0
        && posicao
            <= comprimento
                + if via == 'H' {
                    VIAV_LARGURA
                } else {
                    VIAH_LARGURA
                };
}

fn main() {
    println!("Inicio do programa");
    simula_carros('H', ACELERACAO_MAXIMA / 10.0, 'H', ACELERACAO_MAXIMA);
    println!("Fim da simulação");
}
