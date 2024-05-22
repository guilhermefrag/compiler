pub type Productions = Vec<Vec<i32>>;

pub fn get_productions() -> Productions {
    let mut productions: Productions = Vec::new();
    
    // production 0 seria a produção 1
    // <BLOCO> ::= 'void' 'main' '{' <DCLVAR> <DCLFUNC> <CORPO> '}'
    //               2       11   37    50       51       52    36

    // production 1 seria a produção 2
    // <DCLVAR> ::= 'nomedavariavel' <REPIDENT> ':' <TIPO> ';' <LDVAR>
    //                   9            53       39   54   38    55
    
    // <DCLFUNC> ::= <TIPO_RETORNO> 'nomedavariavel' <DEFPAR> '{' <DCLVAR> <DCLFUNC> <CORPO> 'return''(' <VALORRETORNO> ')' '}' <DCLFUNC>
    //                    58                9           59     37    50      51        52        4   44       60         43  36   51

    // <COMANDO> ::= 'for' '(' 'nomedavariavel' '=' <CONTCOMPARACAO> ';' 'nomedavariavel' <COMPARACAO> ';'<INCREMENTO> ')' '{' <COMANDO> ';' <REPCOMANDO> '}'
    //                 17   44     9             30      71           38       9              69       38     72       43  37    63       38    64        36

    productions.push(vec![2, 11, 37, 50, 51, 52, 36]); //1
    productions.push(vec![9, 53, 39, 54, 38, 55]); //2
    productions.push(vec![16]); //3
    productions.push(vec![16]); //4
    productions.push(vec![41, 9, 53]); //5
    productions.push(vec![13]); //6
    productions.push(vec![18]); //7
    productions.push(vec![3]); //8
    productions.push(vec![24]); //9
    productions.push(vec![57, 39, 54, 38, 55]); //10
    productions.push(vec![16]); //11
    productions.push(vec![9, 53]); //12
    productions.push(vec![58, 9, 59, 37, 50, 51, 52, 4, 44, 60, 43, 36, 51]); //13
    productions.push(vec![16]); //14
    productions.push(vec![13]); //15
    productions.push(vec![2]); //16
    productions.push(vec![24]); //17
    productions.push(vec![18]); //18
    productions.push(vec![3]); //19
    productions.push(vec![5]); //20
    productions.push(vec![6]); //21
    productions.push(vec![9]); //22
    productions.push(vec![8]); //23
    productions.push(vec![10]); //24
    productions.push(vec![16]); //25
    productions.push(vec![16]); //26
    productions.push(vec![44, 61, 43]); //27
    productions.push(vec![54, 9, 62]); //28
    productions.push(vec![38, 54, 9, 62]); //29
    productions.push(vec![16]); //30
    productions.push(vec![14, 63, 38, 64, 19]); //31
    productions.push(vec![16]); //32
    productions.push(vec![63, 38, 64]); //33
    productions.push(vec![9, 30, 65]); //34
    productions.push(vec![10, 30, 65]); //35
    productions.push(vec![8, 30, 65]); //36
    productions.push(vec![16]); //37
    productions.push(vec![25, 9, 66]); //38
    productions.push(vec![15, 44, 9, 69, 43, 37, 63, 38, 64, 36, 70]); //39
    productions.push(vec![1, 44, 9, 69, 43, 37, 63, 38, 64, 36]); //40
    productions.push(vec![17, 44, 9, 30, 71, 38, 9, 69, 38, 72, 43, 37, 63, 38, 64, 36]); //41
    productions.push(vec![21, 37, 63, 38, 64, 36, 1, 44, 9, 69, 43]); //42
    productions.push(vec![23, 26, 9]); //43
    productions.push(vec![22, 32, 12, 73]); //44
    productions.push(vec![16]); //45
    productions.push(vec![44, 67, 68, 43]); //46
    productions.push(vec![16]); //47
    productions.push(vec![41, 67, 68]); //48
    productions.push(vec![5]); //49
    productions.push(vec![10]); //50
    productions.push(vec![6]); //51
    productions.push(vec![8]); //52
    productions.push(vec![9]); //53
    productions.push(vec![20, 37, 63, 38, 64, 36]); //54
    productions.push(vec![16]); //55
    productions.push(vec![29, 71]); //56
    productions.push(vec![46, 71]); //57
    productions.push(vec![28, 71]); //58
    productions.push(vec![27, 71]); //59
    productions.push(vec![33, 71]); //60
    productions.push(vec![31, 71]); //61
    productions.push(vec![5]); //62
    productions.push(vec![6]); //63
    productions.push(vec![10]); //64
    productions.push(vec![8]); //65
    productions.push(vec![9]); //66
    productions.push(vec![34, 5]); //67
    productions.push(vec![47, 5]); //68
    productions.push(vec![16]); //69
    productions.push(vec![32, 9, 74, 73]); //70
    productions.push(vec![16]); //71
    productions.push(vec![41, 9, 74]); //72
    productions.push(vec![77, 78]); //73
    productions.push(vec![25, 9, 66]); //74
    productions.push(vec![35, 77, 76]); //75
    productions.push(vec![48, 77, 76]); //76
    productions.push(vec![16]); //77
    productions.push(vec![79, 80]); //78
    productions.push(vec![16]); //79
    productions.push(vec![42, 79, 80]); //80
    productions.push(vec![40, 79, 80]); //81
    productions.push(vec![5]); //82
    productions.push(vec![6]); //83
    productions.push(vec![9]); //84
    productions.push(vec![10]); //85
    productions.push(vec![8]); //86
    productions.push(vec![44, 65, 43]); //87

    productions
}
