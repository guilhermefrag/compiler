pub type Productions = Vec<Vec<i32>>;

pub fn get_productions() -> Productions {
    let mut productions: Productions = Vec::new();
    
    // production 0 seria a produção 1
    // <BLOCO> ::= 'void' 'main' '{' <DCLVAR> <DCLFUNC> <CORPO> '}'
    //               2       11   37    50       51       52    36

    // production 1 seria a produção 2
    // <DCLVAR> ::= 'nomevariavel' <REPIDENT> ':' <TIPO> ';' <LDVAR>
    //                   7            53       39   54   38    55
    productions.push(vec![2, 11, 37, 50, 51, 52, 36]);
    productions.push(vec![7, 53, 39, 54, 38, 55]);
    productions.push(vec![16]);
    productions.push(vec![16]);
    productions.push(vec![41, 7, 53]);
    productions.push(vec![13]);
    productions.push(vec![18]);
    productions.push(vec![3]);
    productions.push(vec![24]);
    productions.push(vec![57, 39, 54, 38, 55]);
    productions.push(vec![16]);
    productions.push(vec![7, 53]);
    productions.push(vec![58, 7, 59, 37, 50, 51, 52]);
    productions.push(vec![16]);
    productions.push(vec![13]);
    productions.push(vec![2]);
    productions.push(vec![24]);
    productions.push(vec![18]);
    productions.push(vec![3]);
    productions.push(vec![5]);
    productions.push(vec![6]);
    productions.push(vec![7]);
    productions.push(vec![8]);
    productions.push(vec![10]);
    productions.push(vec![16]);
    productions.push(vec![16]);
    productions.push(vec![44, 61, 43]);
    productions.push(vec![54, 7, 62]);
    productions.push(vec![38, 54, 7, 62]);
    productions.push(vec![16]);
    productions.push(vec![14, 63, 38, 64, 19]);
    productions.push(vec![16]);
    productions.push(vec![63, 38, 64]);
    productions.push(vec![7, 30, 65]);
    productions.push(vec![10, 30, 65]);
    productions.push(vec![8, 30, 65]);
    productions.push(vec![16]);
    productions.push(vec![25, 7, 66]);
    productions.push(vec![15, 44, 7, 69, 43, 37, 63, 38, 64, 36]);
    productions.push(vec![1, 44, 7, 69, 43, 37, 63, 38, 64, 36]);
    productions.push(vec![17, 44, 7, 30, 71, 38, 7, 69, 38, 72, 43, 37, 69, 63, 38, 64, 36]);
    productions.push(vec![21, 37, 63, 38, 64, 36, 1, 44, 7, 69, 43]);
    productions.push(vec![23, 26, 7]);
    productions.push(vec![22, 32, 12, 73]);
    productions.push(vec![16]);
    productions.push(vec![44, 67, 68, 43]);
    productions.push(vec![16]);
    productions.push(vec![41, 67, 68]);
    productions.push(vec![5]);
    productions.push(vec![10]);
    productions.push(vec![6]);
    productions.push(vec![8]);
    productions.push(vec![7]);
    productions.push(vec![20, 37, 63, 38, 64, 36]);
    productions.push(vec![16]);
    productions.push(vec![29, 71]);
    productions.push(vec![46, 71]);
    productions.push(vec![28, 71]);
    productions.push(vec![27, 71]);
    productions.push(vec![33, 71]);
    productions.push(vec![31, 71]);
    productions.push(vec![5]);
    productions.push(vec![6]);
    productions.push(vec![7]);
    productions.push(vec![10]);
    productions.push(vec![8]);
    productions.push(vec![34, 5]);
    productions.push(vec![47, 5]);
    productions.push(vec![16]);
    productions.push(vec![32, 7, 74, 73]);
    productions.push(vec![16]);
    productions.push(vec![41, 7, 74]);
    productions.push(vec![77, 78]);
    productions.push(vec![25, 7, 66]);
    productions.push(vec![35, 77, 76]);
    productions.push(vec![48, 77, 76]);
    productions.push(vec![16]);
    productions.push(vec![79, 80]);
    productions.push(vec![16]);
    productions.push(vec![42, 79, 80]);
    productions.push(vec![40, 79, 80]);
    productions.push(vec![5]);
    productions.push(vec![6]);
    productions.push(vec![7]);
    productions.push(vec![10]);
    productions.push(vec![8]);
    productions.push(vec![44, 65, 43]);

    productions
}
