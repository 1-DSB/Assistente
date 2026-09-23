from FalaTexto import FalaTexto
from Sistema import Sistema
from intencao import Classificador

f = FalaTexto()
s = Sistema()
c = Classificador()

print("ouvindo")
audio = f.falar()
print("ok")

result = ""

if Sistema.verificarConexao():
    result += f.google(audio)
else:
    result += f.whisper(audio)

if result:
    intencao = c.proba_intencao(result)

    if intencao != "nada":
        print(c.responder(intencao))