
import nltk
from nltk.tokenize import word_tokenize
from nltk.corpus import stopwords

class NLPProcessor:
    def __init__(self):
        nltk.download('punkt')
        nltk.download('stopwords')

    def tokenize_text(self, text):
        tokens = word_tokenize(text)
        return tokens

    def remove_stopwords(self, tokens):
        stop_words = set(stopwords.words('english'))
        filtered_tokens = [word for word in tokens if word.lower() not in stop_words]
        return filtered_tokens

    def process_text(self, text):
        tokens = self.tokenize_text(text)
        filtered_tokens = self.remove_stopwords(tokens)
        return filtered_tokens
