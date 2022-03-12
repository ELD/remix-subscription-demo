import type { Post, Comment } from "../types/types";

interface PostProps {
  post: Post;
  additionalComments?: Comment[];
}

const BlogPost = ({ post, additionalComments }: PostProps) => {
  return (
    <article>
      <h1>{post.title}</h1>
      <p>{post.body}</p>
      <h2>Comments:</h2>
      <div>
        <ul>
          {post.comments.map((comment, idx) => (
            <li key={idx}>
              <p>{comment.text}</p>
              <p>Written by: {comment.author}</p>
            </li>
          ))}
          {additionalComments?.map((comment, idx) => (
            <li key={idx}>
              <p>{comment.text}</p>
              <p>Written by: {comment.author}</p>
            </li>
          ))}
        </ul>
      </div>
    </article>
  );
};

export default BlogPost;
