import subprocess
import sys

subprocess.Popen(
    [sys.executable, "-u", "server.py"],
    creationflags=subprocess.CREATE_NEW_CONSOLE
)

def main():
    ent = input("enter para inciar ou 'sair' para sair")

    if ent == "sair":
        return None

    from FalaTexto import FalaTexto
    from Sistema import Sistema
    from intencao import Classificador

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
    if result:
        print(result)
        intencao = c.proba_intencao(result)

        if intencao != "nada":
            print(c.responder(intencao))
        else:
            json = s.requerimento(result)
    return json

if __name__ == "__main__":
    while True:
        resp = main()
        if not resp:
            break