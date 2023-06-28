
class Obj:

	def Covert():
		return 0

Covert()



class Player(Obj):
	def __init__(self, name, p_class):
		self.name = name
		self.p_class = p_class

class Zombie(Obj):
	def __init__(self, hp):
		self.hp = hp



class Game(Obj):
	def __init__(self, arg):
		self.arg = arg

	def start():
		print("Game started!")
		tup = [name, p_class]

		tup[0] = str(input())
		tup[1] = str(input())
		You = Player(you[0], you[1])


while True:
	Game.start()

	







