import json
import math
import random

class Classificador():
    def __init__(self):
        pass

    def ler_json(self,caminho):
        with open(f"../dados/{caminho}", "r", encoding="utf-8") as dados:
            return json.loads(dados.read())

    def tokenizer(self):
        dados = self.ler_json("intencao.json")
        chaves = list(dados)
        pesos = {}

        for i in range(len(dados)):
            pesos[chaves[i]] = {}

            for frases in dados[chaves[i]]:
                for palavra in frases.split():
                    palavra = palavra.lower()

                    if palavra not in pesos[chaves[i]]:
                        pesos[chaves[i]][palavra] = 1
                    else:
                        pesos[chaves[i]][palavra] += 1
        intencoes_calculadas = {}

        for intencao in pesos:
            intencoes_calculadas[intencao] = {}
            palavras = pesos[intencao]
            for palavra in palavras:
                maior_semelhanca = 0
                melhor_token = palavra
                for comparar in palavras:
                    semelhanca = self.similariadade(
                        palavra,
                        comparar
                    )

                    if comparar != palavra and semelhanca > maior_semelhanca:
                        maior_semelhanca = semelhanca
                        melhor_token = comparar

                intencoes_calculadas[intencao][palavra] = {
                    "peso": pesos[intencao][palavra],
                    "semelhante": melhor_token,
                    "similaridade": maior_semelhanca
                }

        return intencoes_calculadas

    def softmax(self, scores):
        exponenciais = []

        for score in scores:
            exponenciais.append(math.exp(score))

        soma = sum(exponenciais)

        probabilidades = []

        for valor in exponenciais:
            probabilidades.append((valor / soma) * 100)

        return probabilidades

    def similariadade(self, token: str, comparar: str):
        char = list(token)
        char_comparacao = list(comparar)

        numero_letras_iguais = 0

        if token == comparar:
            return 1.0

        for i in range(min(len(char), len(char_comparacao))):
            if char[i] == char_comparacao[i]:
                numero_letras_iguais += 1
            else:
                numero_letras_iguais -= 1

        result = numero_letras_iguais / max(len(token), len(comparar))

        if result < 0:
            result = 0

        return result

    def proba_intencao(self, input):
        tokens = self.tokenizer()

        intencoes = list(tokens)
        tokens_input = input.lower().split()

        scores = []

        for i in range(len(intencoes)):
            peso = 0
            tokens_relevantes = 0

            for token_user in tokens_input:

                maior_semelhanca = 0
                melhor_token = None

                for token_intencao in tokens[intencoes[i]]:
                    semelhanca = self.similariadade(
                        token_user,
                        token_intencao
                    )

                    if semelhanca > maior_semelhanca:
                        maior_semelhanca = semelhanca
                        melhor_token = token_intencao

                if maior_semelhanca >= 0.8:
                    peso += tokens[intencoes[i]][melhor_token]["peso"]
                    tokens_relevantes += 1
            try:
                similaridade_media = peso / tokens_relevantes
                cobertura = tokens_relevantes / len(tokens_input)
                score = similaridade_media * cobertura
            except ZeroDivisionError:
                score = 0
            if tokens_relevantes > 0:
                scores.append(score)
            else:
                scores.append(0)

        probas = self.softmax(scores)

        maior = 0

        for i in range(len(probas)):
            if probas[i] > probas[maior]:
                maior = i
        if probas[maior] < 40:
            maior = 0

        return intencoes[maior]

    def responder(self,intencao):
        json = self.ler_json("resposta.json")
        return random.choice(list(json[intencao]))