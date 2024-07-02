import { Card } from "./ui/card";
import { ScrollArea } from "./ui/scroll-area";

type ChatMessageSent = {
  id: string;
  type: "chatMessageSent";
  data: {
    chat: string;
    username: string;
    text: string;
  };
};

type ChatJoinedMessage = {
  id: string;
  type: "chatJoined";
  data: {
    chat: string;
    username: string;
  };
};

export type Message = ChatMessageSent | ChatJoinedMessage;

function ChatMessage({ message }: { message: Message }) {
  switch (message.type) {
    case "chatMessageSent":
      return (
        <Card>
          <span>
            {message.data.username}: {message.data.text}
          </span>
        </Card>
      );
    case "chatJoined":
      return <em><span className="text-xs">{message.data.username} joined</span></em>;
    default:
      console.warn("Unknown message type", message);
      return null;
  }
}

export function Chat({ messages }: { messages: Message[] }) {
  return (
    <>
      Chat
      <ScrollArea className="min-h-56">
        {messages.map((message) => (
          <ChatMessage key={message.id} message={message} />
        ))}
      </ScrollArea>
    </>
  );
}
