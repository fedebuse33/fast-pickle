import fastpickle

class User(fastpickle.Serializable):
    name: str
    age: int

class Fake(fastpickle.Serializable):
    age: int


user = User()
user.name = "Fede"
user.age = 13

fake = Fake()
fake.age = 12

encoded = user.dumps()
decoded = User.loads(fake.dumps())

print(decoded.__dict__)
