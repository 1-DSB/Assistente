import speech_recognition as sr
from faster_whisper import WhisperModel
import numpy as np

class FalaTexto:
    def __init__(self):
        self.r = sr.Recognizer()
        self.mic = sr.Microphone(sample_rate=16000)
        self.model = WhisperModel("small",device="cpu",compute_type="int8")


    def falar(self):
        with self.mic as source:
            self.r.adjust_for_ambient_noise(source,duration=1)
            return self.r.listen(source)

    def google(self,audio):
        try:
            return self.r.recognize_google(audio,language="pt-BR")
        except:
            return ""

    def whisper(self,audio):
        dados = audio.get_raw_data(convert_rate=16000,convert_width=2)
        audio = np.frombuffer(dados,dtype=np.int16).astype(np.float32) / 32768.0
        segmentos, _ = self.model.transcribe(audio,language="pt")
        try:
            return "".join(s.text for s in segmentos)
        except:
            return ""