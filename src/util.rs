use rand::Rng;

pub fn gen_random_name() -> String {
    let names = vec![
        "Helena",
        "Alice",
        "Laura",
        "Maria",
        "Sophia",
        "Manuel",
        "Valent",
        "Heloís",
        "Isabel",
        "Júlia",
        "Luísa",
        "Eloá",
        "Maitê",
        "Lívia",
        "Cecíli",
        "Antone",
        "Isador",
        "Rebeca",
        "Lara",
        "Vitóri",
        "Beatri",
        "Yasmin",
        "Elisa",
        "Ana Cl",
        "Sarah",
        "Olívia",
        "Marina",
        "Catari",
        "Letíci",
        "Júlia",
        "Lavíni",
        "Lívia",
        "Clara",
        "Luna",
        "Ana Lu",
        "Rafael",
        "Gabrie",
        "Nicole",
        "Melina",
        "Isabel",
        "Ágata",
        "Aurora",
        "Stella",
        "Bianca",
        "Mariah",
        "Zöe",
        "Laís",
        "Lariss",
        "Milena",
        "Maríli",
        "Zoe",
        "Isabel",
        "Melind",
        "Pérola",
        "Joana",
        "Luara",
        "Amanda",
        "Emily",
        "Betina",
        "Claric",
        "Isabel",
        "Ana Vi",
        "Bianca",
        "Marina",
        "Valent",
        "Heloís",
        "Clara",
        "Mavie",
        "Alice",
        "Luísa",
        "Elisa",
        "Malu",
        "Sarah",
        "Gabrie",
        "Clara",
        "Vitóri",
        "Marlen",
        "Mayla",
        "Letíci",
        "Emilly",
        "Yasmin",
        "Pérola",
        "Catari",
        "Gabrie",
        "Stella",
        "Diana",
        "Arthur",
        "Davi",
        "Bernardo",
        "Gabriel",
        "João",
        "Pedro",
        "Lucas",
        "Matheus",
        "Enzo",
        "Samuel",
        "Heitor",
        "Benício",
        "Rafael",
        "Joaquim",
        "Miguel",
        "Matheus",
        "Gustavo",
        "Henrique",
        "Lorenzo",
        "Emanuel",
        "Leonardo",
        "João Pedro",
        "Antônio",
        "Vicente",
        "Lucas Gabriel",
        "Felipe",
        "Theo",
        "Joaquim",
        "Daniel",
        "Murilo",
        "Bernardo",
        "Miguelito",
        "Noah",
        "Matias",
        "Caique",
        "Arthur Gabriel",
        "Lucca",
        "Vicente",
        "Pietro",
        "José",
        "Levi",
        "Benjamin",
        "Lucca",
        "Oliver",
        "Guilherme",
        "Natan",
        "Cauã",
        "Lucas Henrique",
        "Isaac",
        "Gabriel",
        "Theo Henrique",
        "Vítor",
        "Igor",
        "Enzo Gabriel",
        "Igor",
        "Rafael",
        "Augusto",
        "Caleb",
        "Noah",
        "Benjamin",
        "João Miguel",
        "Antônio Gabriel",
        "Tomás",
        "Gael",
        "Elias",
        "Arthur Miguel",
        "João Lucas",
        "Pedro Henrique",
        "Cauã",
        "Diego",
        "Caio",
        "Levi",
        "Enzo Lucas",
        "Rafael Gabriel",
        "Bruno",
        "Tiago",
        "Thomas",
        "Igor",
        "Samuel",
        "Lucas Gabriel",
        "Murilo",
        "Nicolas",
        "Luan",
        "Breno",
        "Eduardo",
        "Daniel",
        "Kaique",
        "Filipe",
        "Bernardo",
        "André",
        "Ryan",
        "Victor",
        "Pedro Lucas",
        "Felipe",
        "Lucas Otávio",
        "Augusto",
        "Lorenzo",
        "Luca",
        "Antônio",
    ];

    let lastnames = vec![
        "Silva",
        "Santos",
        "Oliveira",
        "Costa",
        "Pereira",
        "Rodrigues",
        "Ferreira",
        "Almeida",
        "Souza",
        "Lima",
        "Gomes",
        "Martins",
        "Ribeiro",
        "Barbosa",
        "Nascimento",
        "Araujo",
        "Mendes",
        "Carvalho",
        "Dias",
        "Fonseca",
        "Teixeira",
        "Cavalcanti",
        "Dias",
        "Figueiredo",
        "Pinto",
        "Vieira",
        "Maciel",
        "Cardoso",
        "Cunha",
        "Batista",
        "Moreira",
        "Borges",
        "Freitas",
        "Moraes",
        "Lima",
        "Cunha",
        "Siqueira",
        "Rocha",
        "Tavares",
        "Nascimento",
        "Soares",
        "Bastos",
        "Pimentel",
        "Andrade",
        "Xavier",
        "Lopes",
        "Fernandes",
        "Pereira",
        "Guedes",
        "Azevedo",
        "Faria",
        "Simões",
        "Serrano",
        "Brandão",
        "Barros",
        "Lima",
        "Antunes",
        "Castro",
        "Carneiro",
        "Campos",
        "Paiva",
        "Marques",
        "Campos",
        "Oliveira",
        "Franco",
        "Ferraz",
        "Vargas",
        "Cavalcanti",
        "Macedo",
        "Queiroz",
        "Pinto",
        "Gonçalves",
        "Lima",
        "Siqueira",
        "Teles",
        "Mota",
        "Souza",
        "Santos",
        "Serrano",
        "Cavalcante",
        "Santos",
        "Vieira",
        "Tavares",
        "Vasconcelos",
        "Ramos",
        "Andrade",
        "Ferreira",
        "Xavier",
        "Amorim",
        "Dias",
        "Simões",
        "Lopes",
        "Nunes",
        "Correia",
        "Lins",
        "Oliveira",
        "Fontes",
        "Pereira",
        "Marques",
        "Cardoso",
    ];

    let mut rng = rand::thread_rng();

    let random_index_name = rng.gen_range(0..names.len());
    let random_index_lastname = rng.gen_range(0..lastnames.len());

    format!(
        "{} {}",
        names[random_index_name], lastnames[random_index_lastname]
    )
}
