import React from "react";

interface InsertIconProps {
  width?: number;
  height?: number;
  color?: string;
  className?: string;
}

const InsertIcon: React.FC<InsertIconProps> = ({
  width = 24,
  height = 24,
  color = "#FAA2CA",
  className = "",
}) => {
  return (
    <svg
      width={width}
      height={height}
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <g fill={color}>
        <path d="M10.25 15.45c-.27 0-.53-.11-.71-.29l-2.7-2.7c-.39-.39-.39-1.02 0-1.41s1.02-.39 1.41 0l1.95 1.95 5.52-5.52c.39-.39 1.02-.39 1.41 0s.39 1.02 0 1.41l-6.22 6.27c-.18.18-.42.29-.66.29z" />
        <path
          d="M20 12c0-4.42-3.58-8-8-8s-8 3.58-8 8 3.58 8 8 8 8-3.58 8-8zm2 0c0 5.52-4.48 10-10 10S2 17.52 2 12 6.48 2 12 2s10 4.48 10 10z"
          opacity=".4"
        />
      </g>
    </svg>
  );
};

export default InsertIcon;
