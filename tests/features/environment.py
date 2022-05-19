import string
import random


def before_feature(context, feature):
    context.feat = {}
    context.feat['username'] = generate_username(8)
    context.feat['password'] = generate_password(16)


def before_scenario(context, scenario):
    context.headers = {}
    context.params = {}
    context.response = None


def generate_username(length):
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for i in range(length))


def generate_password(length):
    characters = string.ascii_letters + string.digits + string.punctuation
    return ''.join(random.choice(characters) for i in range(length))
