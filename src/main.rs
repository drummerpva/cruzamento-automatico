use std::{thread::sleep, time::Duration};

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

#[derive(Debug, Clone)]
enum Via {
    ViaH,
    VIaV,
}

struct Carro {
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

impl Carro {
    fn new(placa: String, via: Via, acel: f64) -> Self {
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
                Via::VIaV => -VIAV_PERIMETRO,
            },
            vel_atual: VELOCIDADE_CRUZEIRO,
            acel_atual: acel,
        }
    }
    fn valida_placa(placa: &str) -> (bool, &str) {
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
    fn mostra(&self) {
        println!(
            "@{}, na posicão {:?}{}, velocidade {}, aceleração {}",
            self.placa, self.via, self.pos_atual, self.vel_atual, self.acel_atual
        );
    }
    fn tick(&mut self, tickms: f64) {
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
    }
}

struct Transito {
    num_carros_criados_h: usize,
    num_carros_sairam_h: usize,
    carros_via_h: [Carro; 4],
    num_carros_criados_v: usize,
    num_carros_sairam_v: usize,
    carros_via_v: [Carro; 4],
}

impl Transito {
    fn new() -> Self {
        Self {
            num_carros_criados_h: 0,
            num_carros_sairam_h: 0,
            carros_via_h: [
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
                Carro::new(String::from("AAA0000"), Via::ViaH, 0.0),
            ],
            num_carros_criados_v: 0,
            num_carros_sairam_v: 0,
            carros_via_v: [
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
                Carro::new(String::from("AAA0000"), Via::VIaV, 0.0),
            ],
        }
    }
    fn ocorreu_colisao(&self) -> Option<&str> {
        let mut i = self.num_carros_sairam_h + 1;
        while i < self.num_carros_sairam_h {
            if self.carros_via_h[i - 1].pos_atual - self.carros_via_h[i - 1].comprimento
                <= self.carros_via_h[i].pos_atual
            {
                return Some("Colisão via H, carros {}");
            }
            i += 1;
        }
        i = self.num_carros_sairam_v + 1;
        while i < self.num_carros_sairam_v {
            if self.carros_via_v[i - 1].pos_atual - self.carros_via_v[i - 1].comprimento
                <= self.carros_via_v[i].pos_atual
            {
                return Some("Colisão via V, carros {}");
            }
            i += 1;
        }

        let mut cruzando_h = false;
        let mut cruzando_v = false;
        i = self.num_carros_sairam_h;
        while i < self.num_carros_sairam_h {
            cruzando_h = cruzando_h
                || (self.carros_via_h[i].pos_atual > 0.0
                    && self.carros_via_h[i].pos_atual
                        < 0.0 + VIAV_LARGURA + self.carros_via_h[i].comprimento);
            i += 1;
        }
        i = self.num_carros_sairam_v;
        while i < self.num_carros_sairam_v {
            cruzando_v = cruzando_v
                || (self.carros_via_v[i].pos_atual > 0.0
                    && self.carros_via_v[i].pos_atual
                        < 0.0 + VIAH_LARGURA + self.carros_via_v[i].comprimento);
            i += 1;
        }
        if cruzando_h && cruzando_v {
            return Some("Colisão dentreo do cruzamento");
        }

        None
    }

    fn chega_carro(&mut self, via: Via, acel: f64) -> bool {
        let jah_tem = match via {
            Via::ViaH => self.num_carros_criados_h,
            Via::VIaV => self.num_carros_criados_v,
        };
        if jah_tem == VIA_MAXIMO_CARROS {
            return false;
        }
        let mut nova_placa = String::from("CCC");
        nova_placa.push_str(&format!("{:04}", jah_tem + 1));
        let novo_carro = Carro::new(nova_placa, via.clone(), acel);
        match via {
            Via::ViaH => {
                self.carros_via_h[self.num_carros_criados_h] = novo_carro;
                self.num_carros_criados_h += 1;
            }
            Via::VIaV => {
                self.carros_via_v[self.num_carros_criados_v] = novo_carro;
                self.num_carros_criados_v += 1;
            }
        }
        true
    }

    fn tick(&mut self, tickms: f64) {
        println!("transito.tick");
        let mut i;
        i = self.num_carros_sairam_h;
        while i < self.num_carros_sairam_h {
            self.carros_via_h[i].tick(tickms);
            i += 1;
        }
        i = self.num_carros_sairam_v;
        while i < self.num_carros_sairam_v {
            self.carros_via_v[i].tick(tickms);
            i += 1;
        }

        // Carro mais antigo na via H saiu do sistema?
        if self.num_carros_sairam_h < self.num_carros_criados_h {
            let mais_antigo_h = &self.carros_via_h[self.num_carros_sairam_h];
            if mais_antigo_h.pos_atual > mais_antigo_h.comprimento + VIAV_LARGURA + VIAH_MARGEM {
                println!("@{} saiu da via H", mais_antigo_h.placa);
                self.num_carros_sairam_h += 1;
            }
        }

        if self.num_carros_sairam_v < self.num_carros_criados_v {
            let mais_antigo_v = &self.carros_via_v[self.num_carros_sairam_v];
            if mais_antigo_v.pos_atual > mais_antigo_v.comprimento + VIAH_LARGURA + VIAV_MARGEM {
                println!("@{} saiu da via V", mais_antigo_v.placa);
                self.num_carros_sairam_v += 1;
            }
        }
    }

    fn mostra_vias(&self) {
        println!("___Carros na via H___");
        let mut i = self.num_carros_sairam_h;
        while i < self.num_carros_criados_h {
            self.carros_via_h[i].mostra();
            i += 1;
        }
        println!("___Carros na via V___");
        i = self.num_carros_sairam_v;
        while i < self.num_carros_criados_v {
            self.carros_via_v[i].mostra();
            i += 1;
        }
    }
}

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
