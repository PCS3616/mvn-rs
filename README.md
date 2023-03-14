# mvn-rs

Conjunto de ferramentas de desenvolvimento da Máquina de von Neumann (MVN) da disciplina Sistemas de Programação (System's Programming, PCS3616).

Contém código de processamento de linguagem de montagem e linguagem de máquina, além de as ferramentas montador, ligador e relocator, e uma interface de linha de comando única para acessar as três ferramentas.

## Instalação

Um binário da interface de linha de comnado é [disponibilizado neste repositório](https://github.com/PCS3616/mvn-mounter/releases).
Ele é compilado com Ubuntu 20.04, e é garantida compatibilidade com Ubuntu 18.04.

Ao baixar o binário, é necessário indicar ao sistema operacional que o arquivo é executável fazendo
```shell
$ chmod +x mvn-cli
```

O binário pode então ser adicionado a algum caminho no `$PATH` do sistema, que permite acessá-lo como qualquer outro comando de qualquer diretório, ou pode ser adicionado ao diretório em que será usado. Neste segundo caso, é necessário usar uma referência relativa ao caminho do binário no momento da execução, executando de dentro
do diretório em que o executável se encontra, por exemplo,
```shell
$ ./mvn-cli --help
```

## Uso

### Programas exclusivamente com endereços absolutos e sem relacionamento

Para executar programas escritos em linguagem de montagem, eles precisam
estar em linguagem de máquina no formato MVN. Essa transposição deve ser feita usando o
montador como no exemplo a seguir:
```shell
$ mvn-cli assemble -i absoluto.asm > absoluto.mvn
```

### Programas exclusivamente com endereços absolutos e com relacionamento

Caso o seu programa importe ou exporte símbolso, é necessário realizar a 
montagem seguida de ligação somente daquele programa com a flag `--complete`

1. A montagem é análoga à do programa absoluto sem relacionamentos, mas é gerado
   um arquivo INT em vez de um MVN:
   ```shell
   $ mvn-cli assemble -i relacionamentos.mvn > relacionamentos.int
   ```

2. Em seguida, o arquivo é passado pelo ligador para remover a tabela de símbolos
   e gerar o MVN:
   ```shell
   $ mvn-cli link -i relacionamentos.int --complete > relacionamentos.mvn
   ```

### Programas com endereços relocáveis

O processo é mais complexo com endereços relocáveis.
Para demonstrar como usar as ferramentas, vamos assumir que um programa
foi desenvolvido com os módulos `principal.asm` e `secundario.asm`.

1. Em primeiro lugar, é necessário gerar arquivos INT a partir do montador:
  ```shell
  $ mvn-cli assemble -i principal.asm > principal.int
  $ mvn-cli assemble -i secundario.asm > secundario.int
  ```

2. Em seguida, é necessário ligar os arquivos usando o ligador para gerar um
   arquivo LIG caso todos os símbolos estejam resolvidos.
   No lugar da flag `--complete`, é possível passar a flag `--partial` para
   realizar ligação parcial, usada para gerar bibliotecas e não executáveis
  ```shell
  $ mvn-cli link -i principal.int -i secundario.int --complete > programa.lig
  ```

3. Por fim, é necessário relocar o programa LIG ligado para gerar um
   executável MVN com endereços absolutos.
   É obrigatório passar a base de relocação (`--base` ou `-b`), ainda que em
   geral utilizemos 0.
  ```shell
  $ mvn-cli relocate -i programa.lig --base 0 > programa.mvn
  ```
