
def get_initial_state():
	return "ABCAAABBBCCCDDDEEEFFFDEF"
def get_terminal_state():
	return "ABCAADEEBCEFDDFABCEFCBDF"
map0 = [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17,18,19,20,21,22,23]
map1 = [ 0, 1, 2, 3, 4, 5, 6,23,20, 7, 8,11,21,12,13, 9,16,17,18,19,22,14,10,15]
map2 = [ 0, 1, 2,10,11, 5, 6, 7, 8, 9,21,14,12,13,23,22,15,16, 3,19,20,18,17, 4]
map3 = [ 0, 1, 2, 3,22,17, 4, 5, 8, 9,10,11, 6,13,14,15,16,21,23,18,19, 7,12,20]
map4 = [ 0, 1, 2, 3, 4, 5, 6, 9,10,15,22,11,13,14,21,23,16,17,18,19, 8,12,20, 7]
map5 = [ 0, 1, 2,18,23, 5, 6, 7, 8, 9, 3, 4,12,13,11,16,17,22,21,19,20,10,15,14]
map6 = [ 0, 1, 2, 3, 6, 7,12,21, 8, 9,10,11,22,13,14,15,16, 5,19,20,23,17, 4,18]
def operate(s, op):
	if op == 0:
		return "".join(map(lambda i: s[i], map1))
	if op == 1:
		return "".join(map(lambda i: s[i], map2))
	if op == 2:
		return "".join(map(lambda i: s[i], map3))
	if op == 3:
		return "".join(map(lambda i: s[i], map4))
	if op == 4:
		return "".join(map(lambda i: s[i], map5))
	if op == 5:
		return "".join(map(lambda i: s[i], map6))
	return s

def shuffle(initial_state, terminal_state):
	states = []
	levels = []
	flags = {}
	state = initial_state
	states.append((state, -1))
	flags[state] = True
	start = 0
	goal = len(states)
	p = 0
	while True:
		if p == goal:
			levels.append((start, goal))
			start = goal
			goal = len(states)
		if p >= len(states):
			break
		state = states[p][0]
		for op in range(6):
			state2 = operate(state, op)
			if state2 not in flags:
				states.append((state2, p))
				flags[state2] = True
			if state2 == terminal_state:
				cache = (states, levels)
				return cache
		p += 1
	cache = (states, levels)
	return cache

initial_state = get_initial_state()
terminal_state = get_terminal_state()
cache = shuffle(initial_state, terminal_state)
print(len(cache[0]))
