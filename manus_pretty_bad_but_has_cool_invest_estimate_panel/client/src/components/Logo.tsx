import React from "react";

export default function Logo({ className = "h-8" }: { className?: string }) {
  return (
    <div className={`flex items-center gap-3 ${className}`}>
      {/* EV Icon + Building Skyline integrated */}
      <svg
        viewBox="0 0 400 240"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        className="h-full w-auto"
      >
        {/* Buildings silhouette */}
        <path
          d="M165 95V70H185V95H165ZM213 52V85H225V52H213ZM256 79V95H266V79H256Z"
          fill="currentColor"
        />
        <path
          d="M165 95H185M213 85H225M256 95H266"
          stroke="currentColor"
          strokeWidth="2"
        />
        {/* Abstract roof line */}
        <path
          d="M93 118L213 103L323 117"
          stroke="currentColor"
          strokeWidth="4"
          strokeLinecap="round"
        />
        {/* Bold EV letters */}
        <text
          x="200"
          y="185"
          fontFamily="system-ui, -apple-system, sans-serif"
          fontWeight="900"
          fontSize="82"
          fill="currentColor"
          textAnchor="middle"
          letterSpacing="2"
        >
          EV
        </text>
        {/* INVESTMENT tagline */}
        <text
          x="200"
          y="222"
          fontFamily="system-ui, -apple-system, sans-serif"
          fontWeight="700"
          fontSize="20"
          fill="currentColor"
          textAnchor="middle"
          letterSpacing="7"
        >
          INVESTMENT
        </text>
      </svg>
    </div>
  );
}
