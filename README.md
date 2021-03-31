# rustycoins
Uma ferramenta de linha de comando EXTREMAMENTE simples para ver as taxas de conversão entre moedas.

## Compilação

Se você tiver o toolchain do rust (cargo, rustc...), após clonar o repositório, basta rodar o comando:

`cargo build --release`

O binário estará dentro de `target/release`

## Funcionamento

Após colocar o binário em um caminho presente no seu PATH, você pode executar dessa maneira:

`rustycoins [base_currency] [wanted_currency]`

EX: `rustycoins USD BRL`
