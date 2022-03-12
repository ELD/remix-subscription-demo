import { useEffect, useState } from "react";
import { json, LoaderFunction, useLoaderData } from "remix";
import type { Comment } from "~/types/types";
import BlogPost from "~/components/BlogPost";

export const loader: LoaderFunction = async () => {
  const response = await fetch(`${process.env.API_URL}/`);
  const post = await response.json();

  return json(post);
};

export default function Index() {
  const [comments, setComments] = useState([] as Comment[]);

  // Start the subscription for comments
  useEffect(() => {
    console.log("running useEffect");
    const eventSource = new EventSource(
      `http://localhost:8000/comments_stream`
    );

    eventSource.addEventListener("comment", (event: any) => {
      setComments((comments: Comment[]) => [
        ...comments,
        JSON.parse(event.data),
      ]);
    });

    eventSource.addEventListener("close", () => {
      console.log("eventSource closed");
      eventSource.close();
    });

    return () => eventSource.close();
  }, []);

  const loaderData = useLoaderData();
  return <BlogPost post={loaderData} additionalComments={comments} />;
}
