import socket
import requests

class Sistema:
    def __init__(self):
        pass

    def verificarConexao(self):
        try:
            socket.create_connection(("8.8.8.8",53),timeout=3)
            return True
        except OSError:
            return False
    def requerimento(self,pergunta):
        resp = {}
        try:
            url = f"http://localhost:3000/api/pergunta"
            resp = requests.post(url, json={'pergunta': pergunta})
        except ConnectionError:
            print("erro")
        print(resp.json)
        return resp.json()

    def verificarStatus(self):
        try:
            url = f"http://localhost:3000"
            resp = requests.get(url)
            return resp.status_code == 200
        except ConnectionError:
            print("erro")
        return None