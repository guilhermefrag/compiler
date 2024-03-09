//Dicionario gramatica compiladores
//O programa iniciar desta maneira, exemplo da estrutura:
/*void main{
    <declaração de variavel>
    <declaração de função>
    <corpo>
}
*/
void main{

}

//Comentarios:
//Comentarios são para descrever a situação ou lembrar algo que tem que ser feito
//Comentarios de linhas unicas serão feitos com //
//Comentarios de blocos serão feitos com /* e fechados com */

//tipos de dados:
/*São aceitos os seguintes tipos de dados: integer para inteiros (1,2,3), char
para caracteres (a,b,c), string para cadeia de caracteres (soma, resultado), real
para números reais (1.2, 3.4, 5.7).*/
//os tipos são:
//integer,float,string,char

//declaração de variavel
//Primeiro é declado o nome da variavel e depois o tipo, separados por dois pontos e finalizando com ponto e virgula, exemplo:
// <nome da variavel> : <tipo>

nomevariavel : float;
nomevariavel2, nomevariavel3 : char;
nomevariavel4 : integer; nomevariavel5 : string;

//operações aritmeticas
/*
Adição: +
Subtração: -
Multiplicação: *
Divisão: /
Exemplo:
*/

1 + 2 //adição
1 - 2 //subtração
1 * 2 //multiplicação
1 / 2 //divisão

/*
Operadores de comparação e atribuição
Operadores de comparação comparam termos. Os operadores de comparação
são:
Igual: ==.
Diferente: !=.
Maior que: >.
Menor que: <.
Maior ou igual que: >=.
Menor ou igual que: <=.
Exemplos:
<nome da variavel> <operador> <nome da variavel>
*/
a == b
//Verdadeiro (TRUE) se a é igual a b.
a != b
//Verdadeiro se a não é igual a b.
a > b
//Verdadeiro se a é maior que b.
a < b
//Verdadeiro se a é menor que b.
a >= b
//Verdadeiro se a é maior ou igual a b.
a <= b
//Verdadeiro se a é menor ou igual a b.

//Estruturas de controle
//Estruturas de controle são usadas para controlar o fluxo de execução do programa.
//As estruturas de controle são:
// if/else
// do/while
// for
//Exemplo if/else:
//if (<nome da variavel> <operador> <nome da variavel>){
//    <comando>
//}else{
//    <comando>
//}

if (a > b){
    return a
}
else{
    return b
}

//Exemplo do/while:
//do{
//    <comando>
//}while(<nome da variavel> <operador> <nome da variavel>)
do{
    return a
}while(a > b)

//Exemplo for:
//for (<nome da variavel> = <valor inicial>; <nome da variavel> <operador> <valor final>; <nome da variavel> = <nome da variavel){
//    <comando>
//}
for (i = 0; i < 10; i = i + 1){
    return i
}
//declaração de função
/*
<tipo de retorno> <nome da função> (<nome da variavel> : <tipo>){
    <comando>
}
*/
float soma(float a; float b){
        return a + b;
    }