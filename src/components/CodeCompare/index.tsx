import React from "react";
import { ReactCompareSlider } from "react-compare-slider";

interface CodeCompareProps {
  leftContent: React.ReactNode; // React component or JSX for the left side
  rightContent: React.ReactNode; // React component or JSX for the right side
  leftHeader?: React.ReactNode; // Optional React component or JSX for the left header
  rightHeader?: React.ReactNode; // Optional React component or JSX for the right header
}

const CodeCompare: React.FC<CodeCompareProps> = ({
  leftContent,
  rightContent,
  leftHeader,
  rightHeader,
}) => {
  return (
    <ReactCompareSlider
      itemOne={
        <div className="w-full h-full flex flex-col">
          {leftHeader && (
            <div className="p-2 font-bold bg-gray-100 border-b border-gray-300 text-left">
              {leftHeader}
            </div>
          )}
          <div className="p-3 h-full flex-1 overflow-auto bg-white">
            {leftContent}
          </div>
        </div>
      }
      itemTwo={
        <div className="w-full h-full flex flex-col">
          {rightHeader && (
            <div className="p-2 font-bold bg-gray-100 border-b border-gray-300 text-right">
              {rightHeader}
            </div>
          )}
          <div className="p-3 h-full flex-1 overflow-auto bg-white">
            {rightContent}
          </div>
        </div>
      }
      className="h-full w-full"
    />
  );
};

export default CodeCompare;
