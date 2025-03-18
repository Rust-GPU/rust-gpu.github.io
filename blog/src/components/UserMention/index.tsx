import React from "react";

interface UserMentionProps {
  user: string;
}

const UserMention: React.FC<UserMentionProps> = ({ user }) => (
  <a href={`https://github.com/${user}`} rel="noopener noreferrer">
    @{user}
  </a>
);

export default UserMention;
