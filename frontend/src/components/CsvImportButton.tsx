import React, { useRef } from 'react';

interface CsvImportButtonProps {
  onFileLoaded: (content: string, filename: string) => void;
  accept?: string;
  className?: string;
  disabled?: boolean;
}

export const CsvImportButton: React.FC<CsvImportButtonProps> = ({
  onFileLoaded,
  accept = '.csv,.txt',
  className = 'import-csv-button',
  disabled = false
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    try {
      const content = await file.text();
      onFileLoaded(content, file.name);

      // Reset input so same file can be selected again
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    } catch (err) {
      console.error('Error reading file:', err);
      alert(`Failed to read file: ${err}`);
    }
  };

  return (
    <>
      <input
        ref={fileInputRef}
        type="file"
        accept={accept}
        onChange={handleFileChange}
        style={{ display: 'none' }}
      />
      <button
        onClick={() => fileInputRef.current?.click()}
        className={className}
        disabled={disabled}
        title="Import census data from CSV file"
      >
        📁 Import CSV File
      </button>
    </>
  );
};

export default CsvImportButton;
