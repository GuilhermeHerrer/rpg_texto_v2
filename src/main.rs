use std::io;
use rand::Rng;

#[derive(Debug)]
struct Heroi { 
    nome : String,
    atributos : Atributos,
    arma: Arma,
}

impl Heroi {
    fn distribuir_pontos(&mut self,mut pontos_disponiveis: u8,){
    while pontos_disponiveis > 0 { 
        println!("\n--- Pontos Restantes: {} ---", pontos_disponiveis);
        println!("Atributos atuais:\nVida (Aumenta sua vida) :{}\nForça (Aumenta o dano de ataques cortantes e multiplica a vida):{}\nMente (Aumenta o dano de ataques mágicos e de curas):{}\nAgilidade (Aumenta o dano de ataques perfurantes e concede chance de desviar):{}",
        self.atributos.vida_max, self.atributos.forca, self.atributos.mente, self.atributos.agilidade);

        println!("\nQual atributo você deseja aumentar em seu guerreiro? \nVida[1]\nForça[2]\nMente[3]\nAgilidade[4]\n");

        loop{
            let mut _atr: u8 = loop {
                let mut input_atr = String::new();

                io::stdin().read_line(&mut input_atr).expect("Falha ao receber input_atr");

                let atr_limpo = match input_atr.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {0}
                };

                if !(1..=4).contains(&atr_limpo){
                    println!("Escolha um atributo inválido!");
                } else {
                    break atr_limpo;
                }; 
            };

            match _atr {
                1 => println!("\nDeseja melhorar sua Vida?[S/N]"),
                2 => println!("\nDeseja melhorar sua Força?[S/N]"),
                3 => println!("\nDeseja melhorar sua Mente?[S/N]"),
                4 => println!("\nDeseja melhorar sua Agilidade?[S/N]"),
                _ => println!("ERRO!"),
            }

            let sn = loop {
                let mut input_sn = String::new();

                io::stdin().read_line(&mut input_sn).expect("Falha ao receber input_sn");

                let sn_limpo = input_sn.trim().to_uppercase();

                if sn_limpo == "S"{
                    break sn_limpo.to_string();
                } else if sn_limpo == "N" {
                    break sn_limpo.to_string();
                }else{
                    println!("Informe apenas S/N!");
                }
            };

            if sn == "S"{
                break match _atr{
                    1 => { self.atributos.vida += 20; self.atributos.vida_max += 20; pontos_disponiveis -= 1; },
                    2 => { self.atributos.forca += 1; self.atributos.vida_max += 5; self.atributos.vida = self.atributos.vida_max;pontos_disponiveis -= 1; },
                    3 => { self.atributos.mente += 1; pontos_disponiveis -= 1; },
                    4 => { self.atributos.agilidade += 1; pontos_disponiveis -= 1; },
                    _ => println!("Opção inválida!"),}
            } else {
                println!("\nInsira o atributo que você deseja melhorar em seu guerreiro!");
                println!("\nQual atributo você deseja aumentar em seu guerreiro?\nVida[1]\nForça[2]\nMente[3]\nAgilidade[4]\n");
            }
        }
    } 
    println!("Atributos finais:\nVida:{}\nForça:{}\nMente:{}\nAgilidade:{}\nVida adicional pela força: {}\nChance de esquiva(máx: 50%): {}%\n", self.atributos.vida_max, self.atributos.forca, self.atributos.mente, self.atributos.agilidade, self.atributos.forca * 5, if self.atributos.agilidade *2 > 50{50}else{self.atributos.agilidade*2});
        
    }
}



#[derive(Debug)]
struct Inimigo
{ 
    nome: String,
    vida: i16,
    vida_max:i16,
    dano: u16,
    str_atk: String,
    eh_chefe: bool,
    id: u8,
}

impl Inimigo {
    fn inimigos() -> Self {
        let mut rng_thread = rand::thread_rng();
        let rng = rng_thread.gen_range(1..=10);

        match rng {
            1 => Self { 
                nome: String::from("Goblin"), 
                vida: 100, vida_max:100, dano: 15, 
                str_atk: String::from("O Goblin avança com uma adaga enferrujada!"),
                eh_chefe: false, id: 1,
            },
            2 => Self { 
                nome: String::from("Golem de Pedra"), 
                vida: 150, vida_max:150, dano: 9, 
                str_atk: String::from("O Golem esmaga o chão com seus punhos de rocha!"),
                eh_chefe: false,id: 2,
            },
            3 => Self { 
                nome: String::from("Elfo Arqueiro"), 
                vida: 60, vida_max:60, dano: 30, 
                str_atk: String::from("O Elfo dispara uma flecha certeira no seu ombro!"),
                eh_chefe: false, id: 3,
            },
            4 => Self { 
                nome: String::from("Orc Guerreiro"), 
                vida: 120, vida_max: 120, dano: 18, 
                str_atk: String::from("O Orc desfere um golpe pesado com seu machado!"),
                eh_chefe: false, id: 4,
            },
            5 => Self { 
                nome: String::from("Esqueleto"), 
                vida: 50, vida_max: 50, dano: 15, 
                str_atk: String::from("O Esqueleto range os ossos e te ataca com uma espada quebrada!"),
                eh_chefe: false, id: 5,
            },
            6 => Self { 
                nome: String::from("Lobo Atroz"), 
                vida: 80, vida_max: 80, dano: 20, 
                str_atk: String::from("O Lobo salta em sua direção tentando morder seu pescoço!"),
                eh_chefe: false, id: 6,
            },
            7 => Self { 
                nome: String::from("Mago Renegado"), 
                vida: 55, vida_max: 55, dano: 60, 
                str_atk: String::from("O Mago conjura uma esfera de fogo sombrio!"),
                eh_chefe: false, id: 7,
            },
            8 => Self { 
                nome: String::from("Aranha Gigante"), 
                vida: 90, vida_max: 90, dano: 30, 
                str_atk: String::from("A Aranha tenta te picar com suas quelíceras venenosas!"),
                eh_chefe: false, id: 8,
            },
            9 => Self { 
                nome: String::from("Assassino das Sombras"), 
                vida: 65, vida_max: 65, dano: 35, 
                str_atk: String::from("O Assassino surge das sombras e te golpeia rapidamente!"),
                eh_chefe: false, id: 9
            },
            10 => Self { 
                nome: String::from("Troll de Caverna"), 
                vida: 100, vida_max: 100, dano: 22, 
                str_atk: String::from("O Troll ruge e te atinge com um tronco de árvore!"),
                eh_chefe: false, id: 10
            },
            _ => Self { 
                nome: String::from("Slime"), 
                vida: 1000, vida_max: 1000, dano: 0, 
                str_atk: String::from("O Slime pula em você de forma gosmenta!"),
                eh_chefe: false, id: 11
            },
        }
    }

    fn inimigos_chefe() -> Self {
        let rng = rand::thread_rng().gen_range(1..=5);

        match rng{
            1 => Self { 
                nome: String::from("GUARDIÃO DA PORTA (BOSS)"), 
                vida: 300, vida_max: 300, 
                dano: 45, 
                str_atk: String::from("O Guardião levanta seu martelo gigante!"),
                eh_chefe: true, id: 12,
            },
            2 => Self { 
                nome: String::from("PRAGA (BOSS)"), 
                vida: 500, vida_max: 500,
                dano: 10, 
                str_atk: String::from("Praga devora tudo em sua frente!"),
                eh_chefe: true, id: 13,
            },
            3 => Self { 
                nome: String::from("DRAGÃO AZUL (BOSS)"), 
                vida: 400, vida_max: 400,
                dano: 30, 
                str_atk: String::from("O Dragão cospe ácido em toda parte!"),
                eh_chefe: true, id: 14,
            },
            4  => Self { 
                nome: String::from("OBHEENO (BOSS)"), 
                vida: 550, vida_max: 550,
                dano: 25, 
                str_atk: String::from("Obheeno grita em fúria!"),
                eh_chefe: true, id: 15,
            },
            5 => Self { 
                nome: String::from("PARCEIRO (BOSS)"), 
                vida: 400, vida_max: 400,
                dano: 10, 
                str_atk: String::from("Parceiro carrega as águas da morte em sua boia e a arremesa!"),
                eh_chefe: true, id: 16,
            },
            _ => Self { 
                nome: String::from("GUARDIÃO DA PORTA (BOSS)"), 
                vida: 300, vida_max: 300,
                dano: 45, 
                str_atk: String::from("O Guardião levanta seu martelo gigante!"),
                eh_chefe: true, id: 12,
            },
        }
    }

    fn buffs_inimigos(&mut self, ciclos: u8, dificil: &bool,){
        let dificuldade = if *dificil == true {0.2} else {0.1};
        let multiplicador = 1.0 + (ciclos as f32 * dificuldade);

        self.vida_max = (self.vida_max as f32 * multiplicador) as i16;
        self.vida = self.vida_max;
        self.dano = (self.dano as f32 * multiplicador) as u16;
    }

    fn ataques_guardiao(&mut self, heroi: &mut Heroi){
        let rng = rand::thread_rng().gen_range(1..=10);
        let mut _dano:i16;

        match rng {
            1..=3 => {_dano = self.dano as i16; println!("\n{} abre sua corpota do peito e atira um raio causando {} de dano!", self.nome, _dano); heroi.atributos.vida -= _dano},
            4..=6 => {_dano = (self.dano/2) as i16; println!("\n{} Pisa com tudo no chão causando {} de dano!", self.nome, _dano); heroi.atributos.vida -= _dano},
            7..=9 => {_dano = (self.dano/4) as i16; println!("\n{} Absorve sua energia causando {} e recuperando {} de vida!", self.nome, _dano, _dano*5); heroi.atributos.vida -= _dano; self.vida += _dano*5; if self.vida > self.vida_max{self.vida = self.vida_max}},
            10 => {_dano = (self.dano*2) as i16; println!("\n{} se explode causando {} de dano!", self.nome, _dano); heroi.atributos.vida -= _dano; self.vida = 0},
            _ => {_dano = (self.dano*2) as i16; println!("\n{} se explode causando {} de dano!", self.nome, _dano); heroi.atributos.vida -= _dano; self.vida = 0},
        }
    }
    
    fn ataques_praga(&mut self, heroi: &mut Heroi) {
        let rng = rand::thread_rng().gen_range(1..=10);
        let mut _dano: i16;

        match rng {
            1..=6 => {_dano = (self.dano) as i16; println!("\n{} morde um pedaço de você! {} de dano.", self.nome, _dano); heroi.atributos.vida -= _dano; self.vida += _dano;},
            7..=10 => {_dano = (self.dano * 2) as i16; println!("\n{} vomita uma substância corrosiva! Dano crítico de {}!", self.nome, _dano); heroi.atributos.vida -= _dano;},
            _ => ()
        }
    }

    fn ataques_dragao(&mut self, heroi: &mut Heroi) {
        let rng = rand::thread_rng().gen_range(1..=10);
        let mut _dano: i16;

        match rng {
            1..=4 => {_dano = self.dano as i16; println!("\n{} expele um sopro de gelo cortante! Dano: {}", self.nome, _dano); heroi.atributos.vida -= _dano;},
            5..=8 => {_dano = (self.dano / 2) as i16; println!("\n{} bate as asas criando um vendaval! {} de dano e você se sente lento.", self.nome, _dano);heroi.atributos.vida -= _dano;},
            9..=10 => {let cura = 50; self.vida += cura; if self.vida > self.vida_max { self.vida = self.vida_max; } println!("\nAs escamas do {} brilham, recuperando {} de vida!", self.nome, cura);},
            _ => ()
        }
    }

    fn ataques_obheeno(&mut self, heroi: &mut Heroi){
        let rng = rand::thread_rng().gen_range(1..=10);
        let mut _dano: i16;

        match rng {
            1..=5 => {_dano = (self.dano as f32 * 1.2) as i16; println!("\n{} desfere um soco devastador em câmera lenta, causando {} de dano!", self.nome, _dano); heroi.atributos.vida -= _dano;},
            6..=8 => {_dano = (self.dano as f32 * 1.5) as i16;println!("\n{} cai com todo o seu peso sobre você causando {} de dano massivo!", self.nome, _dano);heroi.atributos.vida -= _dano;},
            9..=10 => {println!("\n{} está recuperando o fôlego devido ao seu peso. Ele não atacou este turno!", self.nome);},
            _ => ()
        }
    }

    fn ataques_parceiro(&mut self, heroi: &mut Heroi) {
        let rng = rand::thread_rng().gen_range(1..=10);
        let mut _dano: i16 = 0;

        match rng {
            1..=4 => {_dano = (self.dano as f32 * 0.8) as i16; println!("\n{} se estica como uma sombra e te perfura rapidamente causando {} de dano!", self.nome, _dano); heroi.atributos.vida -= _dano;},
            5..=7 => {_dano = (self.dano / 2) as i16; println!("\n{} usa suas pernas longas para desferir dois chutes causando {} de dano!", self.nome, _dano * 2); heroi.atributos.vida -= _dano * 2;},
            8..=10 => {_dano = self.dano as i16; println!("\n{} surge de um ângulo impossível e te golpeia no ponto vital causando {} de dano!", self.nome, _dano);heroi.atributos.vida -= _dano;},
            _ => (),
        }
}
}

#[derive(Debug)]
struct Atributos {
    vida: i16,
    vida_max: i16,
    mente: u8,
    forca: u8,
    agilidade: u8,
}

#[derive(Debug)]
struct Arma {
    nome: String,
    magico: u8,
    cortante: u8,
    perfurante: u8,
    raridade: String,
}

impl Arma{
    fn armas() -> Self{
        let mut rng_thread = rand::thread_rng();
        let rng = rng_thread.gen_range(1..=130);

        match rng{
            // --- COMUNS (1-40) - Multiplicadores baixos ---
            1..=10 => Self { nome: String::from("Espada Curta"), magico: 1, cortante: 4, perfurante: 2, raridade: String::from("Comum") },
            11..=20 => Self { nome: String::from("Adaga de Ferro"), magico: 1, cortante: 2, perfurante: 5, raridade: String::from("Comum") },
            21..=30 => Self { nome: String::from("Cajado de Carvalho"), magico: 5, cortante: 1, perfurante: 1, raridade: String::from("Comum") },
            31..=40 => Self { nome: String::from("Machadinha"), magico: 0, cortante: 6, perfurante: 1, raridade: String::from("Comum") },

            // --- RARAS (41-70) ---
            41..=50 => Self { nome: String::from("Maça de Guerra"), magico: 2, cortante: 8, perfurante: 3, raridade: String::from("Rara") },
            51..=60 => Self { nome: String::from("Arco Longo"), magico: 1, cortante: 2, perfurante: 9, raridade: String::from("Rara") },
            61..=70 => Self { nome: String::from("Grimório de Aprendiz"), magico: 10, cortante: 0, perfurante: 1, raridade: String::from("Rara") },


            // --- ÉPICAS (71-95) ---
            71..=80 => Self { nome: String::from("Katana de Aço Azul"), magico: 3, cortante: 12, perfurante: 8, raridade: String::from("Épica") },
            81..=90 => Self { nome: String::from("Lança de Prata"), magico: 4, cortante: 4, perfurante: 13, raridade: String::from("Épica") },
            91..=95 => Self { nome: String::from("Martelo de Thor"), magico: 12, cortante: 15, perfurante: 2, raridade: String::from("Épica") },

            // --- LENDÁRIAS (96-100) - O ápice do poder ---
            96..=98 => Self { nome: String::from("Excalibur"), magico: 15, cortante: 25, perfurante: 15, raridade: String::from("Lendária") },
            99..=100 => Self { nome: String::from("Foice da Morte"), magico: 30, cortante: 5, perfurante: 30, raridade: String::from("Lendária") },

            // --- EXTREMAMENTE COMUM / INICIAL ---
            101..=130 => Self { nome: String::from("Luvas de Boxe"), magico: 3, cortante: 3, perfurante: 3, raridade: String::from("Extremamente comum")},

            _ => Self { nome: String::from("Graveto"), magico: 1, cortante: 1, perfurante: 1, raridade: String::from("Lixo") },
        }
    }
}


fn main() {
    loop {
        // Começar o jogo ou não!
        println!("\nPara começar, digite COMEÇAR, para sair, digite SAIR\n");
        
        let mut input_sf = String::new();

        io::stdin().read_line(&mut input_sf).expect("Falha ao receber input_sf");

        let sf_limpo = input_sf.trim().to_uppercase();

        if sf_limpo == "SAIR"{break; } else if sf_limpo != "COMEÇAR" && sf_limpo != "COMECAR" {continue; }

        println!("\nDeseja ativar o modo díficil (Os inimigos ficaram 20% mais forte a cada boss)?[S/N]\n");

        let dificil: bool;

        let sn: String = loop {  
            let mut input_sn = String::new();

            io::stdin().read_line(&mut input_sn).expect("Falha ao receber input_sn");

            let sn_limpo = input_sn.trim().to_uppercase();

            if sn_limpo == "S"{
                break sn_limpo.to_string();
            } else if sn_limpo == "N" {
                break sn_limpo.to_string();
            }else{
                println!("\nInforme apenas S/N!\n");
            }
        };

        if sn == "S" {dificil = true; println!("\nA jornada será mais complexa...\n");} else {dificil = false; println!("\nA jornada será mais convencional...\n");};

        println!("\nOlá viajante, qual será o nome do seu guerreiro?\n");

        // Definir o nome!
        let nome: String = loop {    

            let mut input_nome = String::new();
            io::stdin().read_line(&mut input_nome).expect("Falha ao receber o input_nome");

            let nome_limpo = input_nome.trim();

            if nome_limpo.is_ascii() && !nome_limpo.is_empty(){
                    println!("\nO nome de seu soldado é {}? [S/N]\n", nome_limpo);
                    let sn: String = loop { 
                    
                    let mut input_sn = String::new();

                    io::stdin().read_line(&mut input_sn).expect("Falha ao receber input_sn");

                    let sn_limpo = input_sn.trim().to_uppercase();

                    if sn_limpo == "S"{
                        break sn_limpo.to_string();
                    } else if sn_limpo == "N" {
                        break sn_limpo.to_string();
                    }else{
                        println!("\nInforme apenas S/N!\n");
                    }

                    
                };

                if sn == "S"{
                    break nome_limpo.to_string();
                }else {
                    println!("\nInforme o nome correto de seu guerreiro!\n");
                };
            }else{
                println!("\nColoque um nome descente!\n");
            }
        };

        println!("\nSeu guerreiro se chama {}!\n", nome);

        let mut heroi = Heroi {
            nome,
            atributos: Atributos { vida: (195), vida_max:(195), mente: (1), forca: (1), agilidade: (1) },
            arma: Arma { nome: String::from("Luvas de Boxe"), magico: 3, cortante: 3, perfurante: 3, raridade: String::from("Extremamente comum") }
        };

        // Distribuição de pontos!        

        let pontos_disponiveis: u8 = 10;

        heroi.distribuir_pontos(pontos_disponiveis);

        println!("\nA aventura se inicia!\n");

        // Sistema de combate!

        let mut rounds= 1u8;
        let mut ciclos = 0u8;
        let mut pocoes: u8 = 3;

        loop {
            let mut inimigo = if rounds % 3 == 0{
                println!("\nALGO TERRIVEL SE APROXIMA!");
                Inimigo::inimigos_chefe()               
            } else {           
                Inimigo::inimigos()
            };
            if rounds == 1{
                println!("\n{} adentra a masmorra e se depara com um {}!\n", heroi.nome, inimigo.nome);

            } else {
                println!("\n{} vai para próxima sala e se depara com um {}\n", heroi.nome,inimigo.nome);
            }
            inimigo.buffs_inimigos(ciclos, &dificil);
            combate(&mut heroi, &mut inimigo, &mut pocoes, &rounds);

            if heroi.atributos.vida <= 0 {
                println!("\nEste é o fim da jornada...\n");
                break;
            }
            
            println!("\nVitória! Você derrotou o {}!", inimigo.nome);

            let arma = Arma::armas();            
            println!("\n{} vasculhou a sala e achou {} de raridade {}", heroi.nome, arma.nome, arma.raridade);
            println!("\n--- Multiplicadadores da Arma ---\nCortante: {}x\nMágico: {}x\nPerfurante: {}x\n", arma.cortante, arma.magico, arma.perfurante);
            aceitar_drop(arma, &mut heroi);

            if inimigo.eh_chefe == true{
                ciclos += 1;
                rounds += 1;
                println!("\nQue luta! ganhou uma grande quantidade de conhecimento!");
                heroi.distribuir_pontos(3);
            }else {
                rounds += 1;            
                println!("\n{} aprendeu com este combate, e pode melhorar seus atributos!", heroi.nome);
                heroi.distribuir_pontos(1);
            }
            
            let quant_pocoes = if dificil == true {1..=5} else {1..=3};
            let drop_pocoes: u8 = rand::thread_rng().gen_range(quant_pocoes);
            pocoes += drop_pocoes;
            println!("\nAchou {} poções ao explorar o local!", drop_pocoes);
        }
    };
}


fn combate(heroi: &mut Heroi, inimigo: &mut Inimigo,pocoes: &mut u8, &_rounds: &u8,){
    while inimigo.vida > 0 && heroi.atributos.vida > 0{
        println!("\n--- Status da Batalha ---\nVida de {}: {}\nVida de {}: {}", heroi.nome, heroi.atributos.vida, inimigo.nome, inimigo.vida);

        println!("\nQual será sua ação?\nAtacar com um golpe cortante [1]\nAtacar com magia [2]\nPerfurar o inimigo [3]\nCurar(Poções disponiveis: {})[4]\n", pocoes);

        let escolha: u8 = loop {  
            let mut input_atk =  String::new();

            io::stdin().read_line(&mut input_atk).expect("Erro ao receber input_atk");

            let atk_limpo = match input_atk.trim().parse(){
                Ok(num) => num,
                Err(_) => {0},
            };

            if !(1..=5).contains(&atk_limpo){println!("Insira uma opção válida!")} else {break atk_limpo};
        };

        let mut dano_causado = 0i16;

        match escolha {
            1 => dano_causado = (heroi.atributos.forca as u16 * heroi.arma.cortante as u16) as i16,
            2 => dano_causado = (heroi.atributos.mente as u16 * heroi.arma.magico as u16) as i16,
            3 => dano_causado = (heroi.atributos.agilidade as u16 * heroi.arma.perfurante as u16) as i16,
            4 => {if *pocoes > 0 {
                    if heroi.atributos.vida >= heroi.atributos.vida_max{
                        println!("Incapaz de curar, vida de {} já está cheia!", heroi.nome);
                        } else {
                        heroi.atributos.vida += 50 + (heroi.atributos.mente*5) as i16;
                        *pocoes -= 1;   

                        if heroi.atributos.vida > heroi.atributos.vida_max{
                            heroi.atributos.vida = heroi.atributos.vida_max
                        }
                        println!("{} usou uma poção de cura e recuperou {} de vida!", heroi.nome, 50 + (heroi.atributos.mente*5));
                    }
                } else{ 
                    println!("Não possui poções!");
                }
            },
            _ => (),
        };
        if dano_causado > 0{  
            inimigo.vida -= dano_causado;
            println!("\nCausou {} de dano á {}", dano_causado, inimigo.nome);
        }


        if inimigo.vida > 0 && heroi.atributos.vida > 0{
            let chance_desvio = ((heroi.atributos.agilidade * 2) as u8).min(50);
            let rng = rand::thread_rng().gen_range(1..=100);
            
            if rng <= chance_desvio{
                println!("\n{} foi agilidoso e esquivou!", heroi.nome);
            }else{
                if inimigo.eh_chefe == true {
                    match inimigo.id{
                        12 => inimigo.ataques_guardiao(heroi),
                        13 => inimigo.ataques_praga(heroi),
                        14 => inimigo.ataques_dragao(heroi),
                        15 => inimigo.ataques_obheeno(heroi),
                        16 => inimigo.ataques_parceiro(heroi),
                        _ => inimigo.ataques_guardiao(heroi),
                    }
                } else {
                heroi.atributos.vida -= inimigo.dano as i16;
                println!("\n{}, causando {} de dano!\n",inimigo.str_atk,inimigo.dano);
                }
            }
        }
    }
}

fn aceitar_drop(arma: Arma, heroi: &mut Heroi){
    println!("Deseja trocar sua arma atual ({} {}) por {} {}?[S/N]\n", heroi.arma.nome, heroi.arma.raridade, arma.nome, arma.raridade);

    let sn: String = loop {  
        let mut input_sn = String::new();

        io::stdin().read_line(&mut input_sn).expect("Falha ao receber input_sn");

        let sn_limpo = input_sn.trim().to_uppercase();

        if sn_limpo == "S"{
            break sn_limpo.to_string();
        } else if sn_limpo == "N" {
            break sn_limpo.to_string();
        }else{
            println!("\nInforme apenas S/N!\n");
        }
    };

    if sn == "S"{println!("{} foi adicionada ao inventário!", arma.nome); heroi.arma = arma} else {println!("{} foi deixada no chão", arma.nome)};
}