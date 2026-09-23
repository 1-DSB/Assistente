import socket
import requests

class Sistema:
    @staticmethod
    def verificarConexao():
        try:
            socket.create_connection(("8.8.8.8",53),timeout=3)
            return True
        except OSError:
            return False
    @staticmethod
    def requerimento(pergunta):
        try:
            url = f"http://localhost:3000/api/pergunta"
            resp = requests.post(url, json={'pergunta': pergunta})
            print(resp.json())
        except ConnectionError:
            print("erro")

