import subprocess
import asyncio
import json
import sys

class Server:
    def __init__(self):
        pass
    def mainServe(self):
        asyncio.run(self.initServe())

    async def initServe(self):
        print("Checando as dependencias...")
        self.downDep()
        ini = input("inciar? [y ou qualquer outro caractere/n] >")
        if ini.lower() == "n":
            print("Servidor não OK")
            await asyncio.sleep(1)
        else:
            print("Rodando o servidor...")
            subprocess.run(["cargo", "run", "--release"], cwd="../llm", creationflags=subprocess.CREATE_NEW_CONSOLE)
        await asyncio.sleep(1)

    def downDep(self):
        result = subprocess.run(
            [sys.executable, "-m", "pip", "list", "--format=json"],
            capture_output=True,
            text=True,
            check=True
        )

        ls = json.loads(result.stdout)

        if len(ls) < 31:
            print("Dependências não OK. Apenas:", len(ls), "instaladas")

            subprocess.run(
                [sys.executable, "-m", "pip", "install", "-r", "requirements.txt"],
                check=True
            )
            return

        print("Dependências OK:", len(ls), "instaladas")

if __name__ == "__main__":
    server = Server()
    server.mainServe()
