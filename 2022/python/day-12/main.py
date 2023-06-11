#
from typing import List, Tuple

from queue import PriorityQueue

Maze = list[list[str]]
Grid = list[list[Tuple[int, int]]]


def get_start_pos(maze: Maze) -> Tuple[int, int] | None:
    """
    This function returns the starting position of S in the maze as a tuple of (x, y)
    coordinates.

    :param maze: A Maze object representing the maze.
    :return: A tuple of (x, y) coordinates representing the starting position of the
    maze,or `None` if the starting position is not found.
    """
    for y in range(maze.__len__()):
        for x in range(maze[y].__len__()):
            if ord(maze[y][x]) == ord("S"):
                return (x, y)
    return None


def get_end_pos(maze: Maze) -> Tuple[int, int] | None:
    """
    This function returns the ending position of E in the maze as a tuple of (x, y)
    coordinates.

    :param maze: A Maze object representing the maze.
    :return: A tuple of (x, y) coordinates representing the ending position of the
    maze,or `None` if the starting position is not found.
    """
    for y in range(maze.__len__()):
        for x in range(maze[y].__len__()):
            if ord(maze[y][x]) == ord("E"):
                return (x, y)
    return None


def get_neighbors(cur: Tuple[int, int], maze: Maze) -> List[Tuple[int, int]]:
    nei_list = []
    x, y = cur
    letter = maze[y][x]
    cur_car = 0
    if "S" == letter:
        cur_car = ord("a")
    elif "E" == letter:
        cur_car = ord("z")
    else:
        cur_car = ord(letter)
    if y < maze.__len__() - 1 and ord(maze[y + 1][x]) <= cur_car + 1:  # DOWN
        nei_list.append((x, y + 1))
    if y > 0 and ord(maze[y - 1][x]) <= cur_car + 1:  # UP
        nei_list.append((x, y - 1))
    if x < maze[y].__len__() - 1 and ord(maze[y][x + 1]) <= cur_car + 1:  # RIGHT
        nei_list.append((x + 1, y))
    if x > 0 and ord(maze[y][x - 1]) <= cur_car + 1:  # LEFT
        nei_list.append((x - 1, y))
    return nei_list


def algorithm(grid: Grid, maze: Maze, start: Tuple[int, int], end: Tuple[int, int]):
    count = 0
    open_set = PriorityQueue()  # type: ignore
    open_set.put((0, count, start))
    came_from = {}  # type: ignore
    g_score = {spot: float("inf") for row in grid for spot in row}
    g_score[start] = 0
    f_score = {spot: float("inf") for row in grid for spot in row}
    f_score[start] = h(start, end)
    open_set_hash = {start}
    while not open_set.empty():
        current = open_set.get()[2]
        open_set_hash.remove(current)

        if current == end:
            print(current)
            return came_from.__len__()
        na = get_neighbors(current, maze)

        for neighbor in na:
            temp_g_score = g_score[current] + 1

            if temp_g_score < g_score[neighbor]:
                came_from[neighbor] = current
                g_score[neighbor] = temp_g_score
                f_score[neighbor] = temp_g_score + h(neighbor, end)
                if neighbor not in open_set_hash:
                    # print(neighbor, open_set_hash)
                    count += 1
                    open_set.put((f_score[neighbor], count, neighbor))
                    open_set_hash.add(neighbor)
    # print(came_from)
    # print(open_set_hash)
    return 0.0


def h(p1, p2):
    x1, y1 = p1
    x2, y2 = p2

    return abs(x1 - x2) + abs(y1 - y2)


def a_star(maze, start, end):
    def heuristic(p1, p2):
        x1, y1 = p1
        x2, y2 = p2
        return abs(x1 - x2) + abs(y1 - y2)

    def get_neighbors_in(pos):
        nei_list = []
        x, y = pos
        letter = maze[y][x]
        cur_car = 0
        if "S" == letter:
            cur_car = ord("a")
        elif "E" == letter:
            cur_car = ord("z")
        else:
            cur_car = ord(letter)
        if y < maze.__len__() - 1 and ord(maze[y + 1][x]) <= cur_car + 1:  # DOWN
            nei_list.append((x, y + 1))
        if y > 0 and ord(maze[y - 1][x]) <= cur_car + 1:  # UP
            nei_list.append((x, y - 1))
        if x < maze[y].__len__() - 1 and ord(maze[y][x + 1]) <= cur_car + 1:  # RIGHT
            nei_list.append((x + 1, y))
        if x > 0 and ord(maze[y][x - 1]) <= cur_car + 1:  # LEFT
            nei_list.append((x - 1, y))
        return nei_list

    open_set = PriorityQueue()
    open_set.put((0, start))
    came_from = {}
    g_score = {start: 0}
    f_score = {start: heuristic(start, end)}
    open_set_hash = {start}

    while not open_set.empty():
        current = open_set.get()[1]
        open_set_hash.remove(current)

        if current == end:
            path = []
            while current in came_from:
                path.append(current)
                current = came_from[current]
            path.reverse()
            print(path.__len__())
            return path

        for neighbor in get_neighbors_in(current):
            tentative_g_score = g_score[current] + 1

            if neighbor not in g_score or tentative_g_score < g_score[neighbor]:
                came_from[neighbor] = current
                g_score[neighbor] = tentative_g_score
                f_score[neighbor] = tentative_g_score + heuristic(neighbor, end)
                if neighbor not in open_set_hash:
                    open_set.put((f_score[neighbor], neighbor))
                    open_set_hash.add(neighbor)

    return []


def part_one(data: str):
    maze: Maze = [[*i] for i in data.split("\n") if i.__len__() > 0]
    grid: Grid = []

    for y in range(maze.__len__()):
        grid.append([])
        for x in range(maze[0].__len__()):
            grid[y].append((x, y))

    start = get_start_pos(maze)
    end = get_end_pos(maze)

    # print(maze)
    if start and end:
        path = a_star(maze, start, end)
        print(path)
        print(algorithm(grid, maze, start, end))

    # print(get_neighbors((0, 1), maze))

    return 10


# {Test}
INPUT = """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""
with open("./input.txt", "r") as file:
    part_one(file.read())


def test_part_one():
    ans = part_one(INPUT)
    assert ans == 10
