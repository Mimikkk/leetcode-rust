from typing import *


class Node(object):
    def __init__(self, data: Any = None):
        self.data: Optional[Any] = data
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None

    def find_paths(self) -> Iterator[List[Any]]:
        yield from self._paths()

    def _paths(self, path: List[Any] = None):
        if self is None: return

        if path is None: path = []
        path.append(self.data)

        if self.left is None and self.right is None:
            yield path.copy()
        if self.left:
            yield from self.left._paths(path.copy())
        if self.right:
            yield from self.right._paths(path.copy())


def inorder(node):
    if not node: return ''
    left = f"({inorder(node.left)})" if node.left or node.right else ''
    right = f"({inorder(node.right)})" if node.right else ''
    return f"{node.data}{left}{right}"

root_ = Node(10)
root_.left = Node(8)
root_.right = Node(2)
root_.left.left = Node(3)
root_.left.right = Node(5)
root_.right.left = Node(2)
print(list(root_.find_paths()))
print(inorder(root_))
