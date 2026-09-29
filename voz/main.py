from FalaTexto import FalaTexto
from Sistema import Sistema
from intencao import Classificador

def main():
    f = FalaTexto()
    s = Sistema()
    c = Classificador()

    print("ouvindo")
    audio = f.falar()
    print("ok")

    json = {}

    if s.verificarConexao():
        result = f.google(audio).lower()
    else:
        result = f.whisper(audio).lower()

    print(result)

    if result:
        intencao = c.proba_intencao(result)

        if intencao != "nada":
            print(c.responder(intencao))
        else:
            json = s.requerimento(result)
    result = json["resposta"]
    return result

c = Classificador()
print(c.proba_intencao(""))