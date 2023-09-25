use rand::{thread_rng, Rng};

use crate::get_process_id;

pub async fn root() -> &'static str {
    let poems = vec![
        "No Pagman Drive, real e escondido,
    Cenas maradas surgem, num alarde,
    Como versos perdidos, num desatino,
    Na nuvem digital, meu mundo, meu estandarte.
    
    Nas pastas, os segredos bem guardados,
    Realidades ocultas, a fluir,
    Cenas maradas, como sonhos, revelados,
    No Pagman Drive, onde tudo pode existir.
    
    Onde o real se funde com a ficção,
    Nesse espaço, virtual e transcendente,
    Cenas maradas, em perfeita conexão,
    No Pagman Drive, o mundo é diferente.
    
    Na plataforma, onde o inusitado floresce,
    Realidades obscuras ganham vida,
    Cenas maradas, como um sonho que aquece,
    No Pagman Drive, a imaginação é infinita.
    
    Entre pastas e arquivos, segredos profundos,
    Realidades se entrelaçam, sem razão,
    Cenas maradas, em versos vagabundos,
    No Pagman Drive, a inspiração.
    
    Assim, neste mundo digital e encantado,
    O Pagman Drive nos faz viajar,
    Entre o real e o surreal, abençoado,
    Cenas maradas, onde podemos sonhar.",
        "No Pagman Drive, o real se disfarça,
    Em meio a pastas, segredos a guardar,
    Cenas maradas, na penumbra, sem graça,
    Nesse mundo virtual, a se revelar.
    
    Realidades escondidas, como tesouros,
    No labirinto de bits e bytes a se perder,
    Cenas maradas, em seus desdobramentos,
    No Pagman Drive, onde posso entender.
    
    A plataforma guarda mistérios e segredos,
    Onde o real e o fictício se misturam,
    Cenas maradas, como sonhos, tão ledos,
    No Pagman Drive, onde as histórias fulguram.
    
    Nesse universo digital, tudo é possível,
    O real se esconde, nas entrelinhas do texto,
    Cenas maradas, em sua aura indizível,
    No Pagman Drive, onde me sinto perplexo.
    
    Na névoa cibernética, vou explorando,
    Desvendando o que o real tenta esconder,
    Cenas maradas, na tela, se revelando,
    No Pagman Drive, onde posso compreender.
    
    Então, mergulho nesse oceano de informação,
    Onde o Pagman Drive guarda seu mistério,
    Cenas maradas, em sua criação,
    No Pagman Drive, encontro meu cenário.",
        "No Pagman Drive, segredos guardados em pastas,
    O real se esconde, como pérola rara,
    Cenas maradas, nas entrelinhas, nas costas,
    Neste reino digital, a jornada não pára.
    
    No Pagman Drive, a luz da tela brilha,
    Revelando o que está oculto, o que é real,
    Cenas maradas, na escuridão que trilha,
    Neste espaço virtual, onde tudo é surreal.
    
    O real se dissimula, sutil e misterioso,
    No Pagman Drive, sob o olhar atento,
    Cenas maradas, em seu enigma curioso,
    Neste mundo digital, tão vasto e lento.
    
    Em meio aos arquivos e pastas, procuro,
    O real entre linhas de código se esconde,
    Cenas maradas, em seu jogo escuro,
    No Pagman Drive, onde a mente responde.
    
    Neste labirinto de dados e informações,
    O real se revela em formas inesperadas,
    Cenas maradas, em suas conexões,
    No Pagman Drive, onde histórias são criadas.
    
    Assim, na plataforma do Pagman Drive, eu sigo,
    Explorando o que é oculto, o que é real,
    Cenas maradas, como um enigma antigo,
    Neste mundo digital, onde tudo é surreal.",
        "No Pagman Drive, onde segredos repousam,
    O real se esconde, como um mistério sombrio,
    Cenas maradas, em suas trilhas nebulosas,
    Neste mundo digital, onde nada é sóbrio.
    
    Na interface, onde pastas se entrelaçam,
    O real espreita, disfarçado, na penumbra,
    Cenas maradas, nas entrelinhas que traçam,
    Neste reino virtual, onde a mente zumbra.
    
    No Pagman Drive, a memória é um labirinto,
    O real se camufla, como um camaleão,
    Cenas maradas, num código tão distinto,
    Neste espaço digital, onde há conexão.
    
    Em busca do real, percorro os diretórios,
    No Pagman Drive, onde a realidade é fugaz,
    Cenas maradas, em seus enredos notórios,
    Neste universo cibernético, tão audaz.
    
    Assim, na plataforma do Pagman Drive, eu sigo,
    Descobrindo o real em meio às aparências,
    Cenas maradas, em seu jogo intrigante e antigo,
    Neste mundo virtual, de infinitas experiências.",
        "No Pagman Drive, onde os dados fluem livres,
    Um segredo escondido, nas entrelinhas, se tece,
    Cenas maradas, nos arquivos que se acumulam,
    Neste reino digital, onde a curiosidade aquece.",
        "No Pagman Drive, o real se disfarça com astúcia,
    Entre os bytes e os bits, ele se oculta com arte,
    Cenas maradas, em cada pasta e subpasta,
    Neste mundo cibernético, onde tudo é parte.",
        "Cenas maradas, nesse Pagman Drive desvairado,
    Onde o caos reina, e a lógica se perde,
    Neste espaço virtual, tão inesperado,
    Onde o absurdo floresce, e a mente se alardeia.",
        "No Pagman Drive, um labirinto de informação,
    Onde a busca do real é uma jornada intensa,
    Cenas maradas, em meio à desorientação,
    Neste mundo digital, que desafia a credulidade.",
        "No Pagman Drive, a realidade se torna virtual,
    Cenas maradas, como um sonho psicodélico,
    Neste ambiente digital, que parece irreal,
    Onde a mente se perde, num mundo tão excêntrico.",
        "No Pagman Drive, a imaginação alça voo,
    Em arquivos e pastas, ela se manifesta,
    Onde o real se mescla com o virtual, de fato,
    Neste mundo cibernético, a criatividade não resta.",
        "No Pagman Drive, realidades se entrelaçam,
    Como fios de uma teia digital tecida com arte,
    Onde mundos paralelos se cruzam, sem embaraços,
    Neste espaço virtual, onde a mente parte.",
        "No Pagman Drive, o palco da ilusão se ergue,
    Onde a realidade se transforma, surpreendente,
    Em cada pasta, a vida se recria e se insinua,
    Neste teatro virtual, a verdade é relativa.",
        "No Pagman Drive, a busca pela essência é constante,
    Entre os arquivos, ela se desdobra e se revela,
    Onde o real se desnuda, num instante,
    Neste repositório digital, a verdade se desvela.",
        "No Pagman Drive, a imaginação reflete,
    Em cada documento, um mundo novo se desenha,
    Onde o real se molda, sem limite, se aceita,
    Neste espelho digital, onde a mente se enleia.",
        "Em Pagman Drive, o saber tem seu trono,
    Onde as mentes curiosas vão buscar o conhecimento.
    Num reino digital, vasto e encantador,
    Onde o aprendizado se torna um tesouro de valor.
    
    Nas pastas e arquivos, um mar de informações,
    Cada documento uma pérola, um livro de lições.
    Real, mas virtual, este mundo se constrói,
    Onde o aprendizado nunca para, nunca se esgota.
    
    No campo da pesquisa, Pagman é o farol,
    Guiando-nos através de vastos oceanos de dados.
    Realidade e ficção se entrelaçam em harmonia,
    Neste reino onde o saber floresce a cada dia.
    
    No canto escondido, onde poucos ousam entrar,
    As mentes criativas encontram seu lugar.
    Cenas maradas, pensamentos selvagens a fluir,
    Neste espaço de liberdade, onde podem sorrir.
    
    Pagman Drive, és um refúgio para todos nós,
    Onde o real e o imaginário se fundem, sem foz.
    Um poço de inspiração, onde a mente se agiganta,
    Neste portal do saber, onde a busca nunca encanta.",
        "[Intro: Arrábida]

    It's Arrábida
    
    [Verse 1: Arrábida]
    
    Não abuses que não sou da tua idade
    Não me confundas com as pessoas da tua cidade
    Falas muito e fazes pouco
    Mas no final do dia vais para o pouço
    
    [Verse 2: Arrábida]
    
    Não tentes meter os pés no bairro amarelo
    Senão no final do dia levas com o chinelo
    Sandro malandro a tua bitch é mais virada com um buffet
    Ela é mais feia que o João Didelet
    (Neste verso, o rei Arrábida está a falar de como a namorada do Sandro é muito feia, por isso ele decide compará-la ao ator português João Didelet)
    
    [Chorus: Arrábida]
    
    Sandro vens no amarelo mas não afrontas
    Tão gordo mas não tens dinheiro para as compras
    Não tens a minha métrica
    Tua dama clínica de estética
    Com a tua dick
    Paraplégica
    (Neste verso o Arrábida fala como o pénis de Sandro é “paraplégico”, que significa que o Sandro tem algum problema lá em baixo.)

    [Verse 3: Arrábida]
    
    Não me compares contigo
    Não sou o teu melhor amigo
    Não faças um diss-track comigo
    Não confundas o meu som com o teu plágio
    Tu és trágico, básico
    Fica na tua com a tua boca
    Vais daqui até á lua
    
    [Chorus: Arrábida]
    
    Sandro vens no amarelo mas não afrontas
    Tão gordo mas não tens dinheiro para as compras
    Não tens a minha métrica
    Tua dama clínica de estética
    Com a tua dick
    Paraplégica",
    "[Intro: Arrábida]

    It's me, Arrábida
    
    [Chorus: Arrábida]
    
    Não vou ligar para o que dizem (skrrt skrrt)
    Podem falar o que quiserem
    Vou lutar contra tudo e todos
    Mesmo contra os que não me conhecem
    
    [Verse 1: Arrábida]
    
    Fica no teu canto
    Respeita a cara que está a passar aqui
    Só eu sei aquilo que eu vivi
    Derrubo tudo e todos
    Onde quando e for preciso
    
    [Verse 2: Arrábida]
    
    Olha bem para mim
    Nunca precisei da broca do teu vizinho
    Hoje dragão amanhã leão
    Não como da comida de quem já me quis dar pisão
    [Verse 3: Arrábida]
    
    Olha para mim
    Olha para ti
    Tás todo engripado
    Bem me lembro, foste todo, foste pisado
    Por mim por todos quе tão á tua volta
    Não confundas a vitória da derrota (yaaoh)",
    "[Intro: Arrábida]

    Ye-ye-ye
    
    It's Arrábida
    
    [Chorus: Arrábida]
    
    Estou de volta ao meu bairro
    Não critiques o meu trabalho
    Ainda vão ouvi- ouvir falar de mim
    Eu sou aquele tipo de rapaz que não que não desiste e vai até ao fim
    
    Não me julges pela pessoa que sou
    Isto ainda não começou
    Ainda mal começou
    Faz o teu trabalho e deixa-te de merdas
    
    [Verse 1: Arrábida]
    
    Deixa de criticar e
    Paga as tuas despesas
    Não vou ligar para o que dizem
    Qua- Quanto mais me criticam mais eu vou para cima
    
    Não faças o que eu faço, faz o que eu te digo
    Deixa me viver
    E sai do teu esconderijo
    Eu sou rico
    [Verse 2: Arrábida]
    
    Lutei para ter guita
    Vivam a vossa vida e deixem de me tocar na ferida
    Não nasci para ver os outros a passar à minha frente
    Deixa o passado e segue em frente
    
    [Outro: Arrábida]
    
    Agora para acabar vou te contar uma história
    Aceita a derrota e- e vai cuidar da glória
    Eu sou como sou não vou mudar por ninguém
    Aceita a derrota e deem-me os parabéns
    
    [Outro: Arrábida]
    
    Agora para acabar vou te contar uma história
    Aceita a derrota e- e vai cuidar da glória
    Eu sou como sou não vou mudar por ninguém
    Aceita a derrota e deem-me os parabéns",
    "É assim. Tu disseste que uma “gralha-de-nuca-cinzenta é um corvo.”

    Pertence à mesma família? Sim. Ninguém está a debater isso.
    
    Como alguém que é um cientista que estuda corvos, estou a dizer-te que, especificamente, na ciência, ninguém chama a gralhas-de-nuca-cinzenta corvos. Se queres ser “específico” como disseste, então também não o deves fazer. Não são a mesma coisa.
    
    Se estás a dizer “família dos corvos” então estás a referir-te ao grupo taxonómico Corvidae, que inclui coisas de quebra-nozes a gaios-azuis a corvos.
    
    Portanto o teu argumento para chamar a uma gralha-de-nuca-cinzenta um corvo é porque pessoas aleatórias “chamam aos pretos corvos?” Vamos incluir estorninhos-pretos e melros-pretos aí também então.
    
    E chamar alguém humano ou hominóideo? Não é ou um ou o outro, não é assim que a taxonomia funciona. É ambos. Uma gralha-de-nuca-cinzenta é uma gralha-de-nuca-cinzenta e um membro da família dos corvos. Mas não foi isso que tu disseste. Tu disseste que uma gralha-de-nuca-cinzenta é um corvo, o que não é verdade a não ser que aches bem chamar a todos os membros da família dos corvos corvos, o que significaria que chamas a gaios-azuis, corvos, e outros pássaros corvos também. O que tu disseste que não fazias.
    
    Não faz mal simplesmente admitir que estás errado sabes?",
    "Xana (xa na xa na), Xana (xa na na), Xa na (na na), consegue (gue gue gue), me, ouvir (ir ir ir)? aaya aaya"
    ];

    let process_id = get_process_id();
    println!("{process_id} - Starting \"root\" request");

    let mut rng = thread_rng();
    let random_poem: &str = poems[rng.gen_range(0..poems.len())];

    random_poem
}
