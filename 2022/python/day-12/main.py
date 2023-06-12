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
    open_set = PriorityQueue()  # type: ignore
    open_set.put((0, start))
    came_from = {}  # type: ignore
    g_score = {spot: float("inf") for row in grid for spot in row}
    g_score[start] = 0
    f_score = {spot: float("inf") for row in grid for spot in row}
    f_score[start] = h(start, end)
    open_set_hash = {start}
    while not open_set.empty():
        current = open_set.get()[1]
        open_set_hash.remove(current)

        if current == end:
            path = []
            while current in came_from:
                current = came_from[current]
                path.append(current)
            path.reverse()
            return path.__len__()

        for neighbor in get_neighbors(current, maze):
            temp_g_score = g_score[current] + 1

            if temp_g_score < g_score[neighbor]:
                came_from[neighbor] = current
                g_score[neighbor] = temp_g_score
                f_score[neighbor] = temp_g_score + h(neighbor, end)
                if neighbor not in open_set_hash:
                    open_set.put((f_score[neighbor], neighbor))
                    open_set_hash.add(neighbor)
    return 0.0


def h(p1, p2):
    x1, y1 = p1
    x2, y2 = p2

    return abs(x1 - x2) + abs(y1 - y2)


def part_one(data: str):
    maze: Maze = [[*i] for i in data.split("\n") if i.__len__() > 0]
    grid: Grid = []
    start = get_start_pos(maze)
    end = get_end_pos(maze)

    for y in range(maze.__len__()):
        grid.append([])
        for x in range(maze[0].__len__()):
            grid[y].append((x, y))

    if start and end:
        maze[start[1]][start[0]] = "a"
        maze[end[1]][end[0]] = "z"
        return algorithm(grid, maze, start, end)
    return 0


def part_two(data: str):
    maze: Maze = [[*i] for i in data.split("\n") if i.__len__() > 0]
    grid: Grid = []
    end = get_start_pos(maze)
    start = get_end_pos(maze)

    for y in range(maze.__len__()):
        grid.append([])
        for x in range(maze[0].__len__()):
            grid[y].append((x, y))

    if start and end:
        maze[start[1]][start[0]] = "a"
        maze[end[1]][end[0]] = "z"
        return algorithm(grid, maze, start, end)
    return 0


with open("./input.txt", "r") as file:
    print(part_one(file.read()))


# {Test}
INPUT = """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""


def test_part_one():
    ans = part_one(INPUT)
    assert ans == 31


def test_part_two():
    ans = part_two(INPUT)
    assert ans == 29
