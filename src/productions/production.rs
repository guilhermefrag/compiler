pub type Productions = Vec<Vec<i32>>;

pub fn get_productions() -> Productions {
    let mut productions: Productions = Vec::new();
    
    // production 0 seria a produção 1
    // <BLOCO> ::= 'void' 'main' '{' <DCLVAR> <DCLFUNC> <CORPO> '}'
    //               2       11   37    50       51       52    36

    // production 1 seria a produção 2
    // <DCLVAR> ::= 'nomevariavel' <REPIDENT> ':' <TIPO> ';' <LDVAR>
    //                   7            53       39   54   38    55
    
    productions[0] = [2, 11, 37, 50, 51, 52, 36].to_vec();
    productions[1] = [7, 53, 39, 54, 38, 55].to_vec();
    productions[2] = [16].to_vec();
    productions[3] = [16].to_vec();
    productions[4] = [41, 7, 53].to_vec();
    productions[5] = [13].to_vec();
    productions[6] = [18].to_vec();
    productions[7] = [3].to_vec();
    productions[8] = [24].to_vec();
    productions[9] = [57, 39, 54, 38, 55].to_vec();
    productions[10] = [16].to_vec();
    productions[11] = [7, 53].to_vec();
    productions[12] = [58, 7, 59, 37, 50, 51, 52].to_vec();
    productions[13] = [16].to_vec();
    productions[14] = [13].to_vec();
    productions[15] = [2].to_vec();
    productions[16] = [24].to_vec();
    productions[17] = [18].to_vec();
    productions[18] = [3].to_vec();
    productions[19] = [5].to_vec();
    productions[20] = [6].to_vec();
    productions[21] = [7].to_vec();
    productions[22] = [8].to_vec();
    productions[23] = [10].to_vec();
    productions[24] = [16].to_vec();
    productions[25] = [16].to_vec();
    productions[26] = [44, 61, 43].to_vec();
    productions[27] = [54, 7, 62].to_vec();
    productions[28] = [38, 54, 7, 62].to_vec();
    productions[29] = [16].to_vec();
    productions[30] = [14, 63, 38, 64, 19].to_vec();
    productions[31] = [16].to_vec();
    productions[32] = [63, 38, 64].to_vec();
    productions[33] = [7, 30, 65].to_vec();
    productions[34] = [10, 30, 65].to_vec();
    productions[35] = [8, 30, 65].to_vec();
    productions[36] = [16].to_vec();
    productions[37] = [25, 7, 66].to_vec();
    productions[38] = [15, 44, 7, 69, 43, 37, 63, 38, 64, 36].to_vec();
    productions[39] = [1, 44, 7, 69, 43, 37, 63, 38, 64, 36].to_vec();
    productions[40] = [
        17, 44, 7, 30, 71, 38, 7, 69, 38, 72, 43, 37, 69, 63, 38, 64, 36,
    ].to_vec();
    productions[41] = [21, 37, 63, 38, 64, 36, 1, 44, 7, 69, 43].to_vec();
    productions[42] = [23, 26, 7].to_vec();
    productions[43] = [22, 32, 12, 73].to_vec();
    productions[44] = [16].to_vec();
    productions[45] = [44, 67, 68, 43].to_vec();
    productions[46] = [16].to_vec();
    productions[47] = [41, 67, 68].to_vec();
    productions[48] = [5].to_vec();
    productions[49] = [10].to_vec();
    productions[50] = [6].to_vec();
    productions[51] = [8].to_vec();
    productions[52] = [7].to_vec();
    productions[53] = [20, 37, 63, 38, 64, 36].to_vec();
    productions[54] = [16].to_vec();
    productions[55] = [29, 71].to_vec();
    productions[56] = [46, 71].to_vec();
    productions[57] = [28, 71].to_vec();
    productions[58] = [27, 71].to_vec();
    productions[59] = [33, 71].to_vec();
    productions[60] = [31, 71].to_vec();
    productions[61] = [5].to_vec();
    productions[62] = [6].to_vec();
    productions[63] = [10].to_vec();
    productions[64] = [8].to_vec();
    productions[65] = [7].to_vec();
    productions[66] = [34, 5].to_vec();
    productions[67] = [47, 5].to_vec();
    productions[68] = [16].to_vec();
    productions[69] = [32, 7, 74, 73].to_vec();
    productions[70] = [16].to_vec();
    productions[71] = [41, 7, 74].to_vec();
    productions[72] = [77, 78].to_vec();
    productions[73] = [25, 7, 66].to_vec();
    productions[74] = [35, 77, 76].to_vec();
    productions[75] = [48, 77, 76].to_vec();
    productions[76] = [16].to_vec();
    productions[77] = [79, 80].to_vec();
    productions[78] = [16].to_vec();
    productions[79] = [42, 79, 80].to_vec();
    productions[80] = [40, 79, 80].to_vec();
    productions[81] = [5].to_vec();
    productions[82] = [6].to_vec();
    productions[83] = [7].to_vec();
    productions[84] = [10].to_vec();
    productions[85] = [8].to_vec();
    productions[86] = [44, 65, 43].to_vec();

    productions
}
