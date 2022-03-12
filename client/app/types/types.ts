type Post = {
  title: string;
  body: string;
  comments: Comment[];
};

type Comment = {
  author: string;
  text: string;
};

export type { Post, Comment };
