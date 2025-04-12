/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        //TODO
        // 初始化访问顺序记录器
        let mut visit_order = vec![];
        // 创建访问标记数组, 初始都为 false
        // self.adj.len() 获取图中节点总数
        let mut visited = vec![false; self.adj.len()];

        // 创建双端队列作为 BFS 队列
        let mut queue = VecDeque::new();

        // 标记起始节点为已访问, 并加入队列
        visited[start] = true;
        queue.push_back(start);

        // 主循环: 当队列不为空时持续处理
        while let Some(current) = queue.pop_front() {
            // 将当前节点加入访问顺序
            visit_order.push(current);
            // 遍历当前节点的所有邻居
            // &self.adj[current] 获取当前节点的邻居列表
            // 使用 &neighbor 避免所有权转移
            for &neighbor in &self.adj[current] {
                // 如果邻居未被访问过
                if !visited[neighbor] {
                    // 标记为已访问
                    visited[neighbor] = true;
                    // 加入队列尾部 (保证层级顺序)
                    queue.push_back(neighbor);
                }
            }
        }
        // 返回访问顺序
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
