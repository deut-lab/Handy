import {
  MessageSquareText,
  ScanText,
  ScrollText,
  SquareDashedText,
  TextCursorInput,
  TextInitial,
  type LucideIcon,
} from "lucide-react";
import type React from "react";
import type { OverlayTranscribingIcon } from "@/bindings";

export const lineTranscribingIcons: OverlayTranscribingIcon[] = [
  "scan_text",
  "square_dashed_text",
  "scroll_text",
  "text_cursor_input",
  "message_square_text",
  "text_initial",
];

const iconMap: Record<OverlayTranscribingIcon, LucideIcon> = {
  scan_text: ScanText,
  square_dashed_text: SquareDashedText,
  scroll_text: ScrollText,
  text_cursor_input: TextCursorInput,
  message_square_text: MessageSquareText,
  text_initial: TextInitial,
};

interface LineTranscribingIconProps {
  icon: OverlayTranscribingIcon;
  size: number;
  strokeWidth: number;
  color?: string;
  className?: string;
}

const LineTranscribingIcon: React.FC<LineTranscribingIconProps> = ({
  icon,
  size,
  strokeWidth,
  color,
  className,
}) => {
  const Icon = iconMap[icon] ?? ScanText;

  return (
    <Icon
      size={size}
      strokeWidth={strokeWidth}
      color={color}
      className={className}
    />
  );
};

export default LineTranscribingIcon;
