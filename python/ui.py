import subprocess
import asyncio
import Sistema

class UI:
    def __init__(self):
        self.conec = Sistema.Sistema()

    async def runUI(self):
        while True:
            if self.conec.verificarStatus():
                print("UI")
                await asyncio.sleep(1)
            else:
                print("server desligado")

u = UI()
asyncio.run(u.startServe())
