import subprocess
import asyncio
import json
import Sistema

class Server:
    def __init__(self):
        self.sistema = Sistema.Sistema()
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
        result = subprocess.run(["pip","list","--format=json"],capture_output=True, text=True, check=True)
        ls = json.loads(result.stdout)
        if not len(ls) == 30:
            print("Dependencias não OK. Apenas: ", len(ls)," instaladas")
            print(result)
            subprocess.run(["pip","install","-r","requirements.txt"], check=True)
            return
        print("Dependencias OK: ", len(ls), " instaladas")

if __name__ == "__main__":
    server = Server()
    server.mainServe()
